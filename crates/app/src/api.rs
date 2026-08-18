use std::{
    future::Future,
    path::PathBuf,
    sync::{Arc, RwLock},
    time::Duration as StdDuration,
};

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use chrono::{DateTime, Duration, Utc};
use memos_api::{Client, Error as ClientError, ResponseValue, auth, types};
use thiserror::Error;
use tokio::runtime::Runtime;
use url::Url;

const REFRESH_BUFFER: Duration = Duration::seconds(30);

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("invalid Memos server URL: {0}")]
    InvalidServerUrl(String),
    #[error("Memos API request failed: {0}")]
    Request(String),
    #[error("Memos response did not include {0}")]
    MissingField(&'static str),
    #[error("network runtime failed: {0}")]
    Runtime(String),
}

#[derive(Clone)]
pub struct ApiSession {
    base_url: String,
    client: Client,
    runtime: Arc<Runtime>,
    access_token_expires_at: Arc<RwLock<Option<DateTime<Utc>>>>,
}

#[derive(Default)]
pub struct MemoDetailData {
    pub attachments: Vec<types::Attachment>,
    pub comments: Vec<types::Memo>,
    pub reactions: Vec<types::Reaction>,
    pub relations: Vec<types::MemoRelation>,
    pub shares: Vec<types::MemoShare>,
}

impl ApiSession {
    pub fn new(server_url: &str, runtime: Arc<Runtime>) -> Result<Self, ApiError> {
        let mut url = Url::parse(server_url.trim())
            .map_err(|error| ApiError::InvalidServerUrl(error.to_string()))?;
        if !matches!(url.scheme(), "http" | "https") || url.host_str().is_none() {
            return Err(ApiError::InvalidServerUrl(
                "expected an http:// or https:// URL with a host".into(),
            ));
        }
        if url.query().is_some() || url.fragment().is_some() {
            return Err(ApiError::InvalidServerUrl(
                "query strings and fragments are not allowed".into(),
            ));
        }

        url.set_query(None);
        url.set_fragment(None);
        let base_url = url.as_str().trim_end_matches('/').to_string();
        let http = reqwest::ClientBuilder::new()
            .connect_timeout(StdDuration::from_secs(10))
            .timeout(StdDuration::from_secs(30))
            .cookie_store(true)
            .user_agent(concat!("memos-desktop/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|error| ApiError::Request(error.to_string()))?;

        Ok(Self {
            client: Client::new_with_client(&base_url, http),
            base_url,
            runtime,
            access_token_expires_at: Arc::new(RwLock::new(None)),
        })
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub async fn instance_profile(&self) -> Result<types::InstanceProfile, ApiError> {
        self.call_without_refresh(|client| async move {
            client.instance_service_get_instance_profile().await
        })
        .await
    }

    pub async fn sign_in_password(
        &self,
        username: String,
        password: String,
    ) -> Result<types::User, ApiError> {
        let session = self.clone();
        let request = types::SignInRequest {
            password_credentials: Some(types::SignInRequestPasswordCredentials {
                username,
                password,
            }),
            sso_credentials: None,
        };

        let response = self
            .call_without_refresh(move |client| async move {
                client.auth_service_sign_in(&request).await
            })
            .await?;

        let token = response
            .access_token
            .ok_or(ApiError::MissingField("access token"))?;
        auth::set_access_token(&session.client, &token)
            .map_err(|error| ApiError::Request(error.to_string()))?;
        *session
            .access_token_expires_at
            .write()
            .expect("access token expiry lock poisoned") = response.access_token_expires_at;

        response
            .user
            .ok_or(ApiError::MissingField("authenticated user"))
    }

    pub async fn sign_in_with_access_token(&self, token: String) -> Result<types::User, ApiError> {
        auth::set_access_token(&self.client, token.trim())
            .map_err(|error| ApiError::Request(error.to_string()))?;
        *self
            .access_token_expires_at
            .write()
            .expect("access token expiry lock poisoned") = None;

        match self.current_user().await {
            Ok(user) => Ok(user),
            Err(error) => {
                auth::clear_access_token(&self.client);
                Err(error)
            }
        }
    }

    pub async fn current_user(&self) -> Result<types::User, ApiError> {
        let response = self
            .call(|client| async move { client.auth_service_get_current_user().await })
            .await?;
        response.user.ok_or(ApiError::MissingField("current user"))
    }

    pub async fn list_memos(
        &self,
        filter: Option<String>,
        archived: bool,
    ) -> Result<Vec<types::Memo>, ApiError> {
        let state = if archived {
            types::MemoServiceListMemosState::Archived
        } else {
            types::MemoServiceListMemosState::Normal
        };
        let response = self
            .call(move |client| async move {
                client
                    .memo_service_list_memos(
                        filter.as_deref(),
                        Some("pinned desc, create_time desc"),
                        Some(100),
                        None,
                        Some(false),
                        Some(state),
                    )
                    .await
            })
            .await?;
        Ok(response.memos)
    }

    pub async fn create_memo(
        &self,
        content: String,
        visibility: types::MemoVisibility,
    ) -> Result<types::Memo, ApiError> {
        let memo = types::Memo {
            attachments: Vec::new(),
            content,
            create_time: None,
            creator: None,
            location: None,
            name: None,
            parent: None,
            pinned: Some(false),
            property: None,
            reactions: Vec::new(),
            relations: Vec::new(),
            snippet: None,
            state: types::MemoState::Normal,
            tags: Vec::new(),
            update_time: None,
            visibility,
        };
        self.call(move |client| async move { client.memo_service_create_memo(None, &memo).await })
            .await
    }

    pub async fn update_memo(
        &self,
        memo: types::Memo,
        update_mask: String,
    ) -> Result<types::Memo, ApiError> {
        let memo_id = resource_id(
            memo.name
                .as_deref()
                .ok_or(ApiError::MissingField("memo name"))?,
        )
        .to_string();
        self.call(move |client| async move {
            client
                .memo_service_update_memo(&memo_id, Some(&update_mask), &memo)
                .await
        })
        .await
    }

    pub async fn delete_memo(&self, memo_name: String) -> Result<(), ApiError> {
        let memo_id = resource_id(&memo_name).to_string();
        self.call(move |client| async move {
            client.memo_service_delete_memo(&memo_id, Some(false)).await
        })
        .await
    }

    pub async fn load_memo_detail(&self, memo_name: String) -> Result<MemoDetailData, ApiError> {
        let memo_id = resource_id(&memo_name).to_string();
        let attachments = self
            .call({
                let memo_id = memo_id.clone();
                move |client| async move {
                    client
                        .memo_service_list_memo_attachments(&memo_id, Some(100), None)
                        .await
                }
            })
            .await?
            .attachments;
        let comments = self
            .call({
                let memo_id = memo_id.clone();
                move |client| async move {
                    client
                        .memo_service_list_memo_comments(
                            &memo_id,
                            Some("create_time asc"),
                            Some(100),
                            None,
                        )
                        .await
                }
            })
            .await?
            .memos;
        let reactions = self
            .call({
                let memo_id = memo_id.clone();
                move |client| async move {
                    client
                        .memo_service_list_memo_reactions(&memo_id, Some(100), None)
                        .await
                }
            })
            .await?
            .reactions;
        let relations = self
            .call({
                let memo_id = memo_id.clone();
                move |client| async move {
                    client
                        .memo_service_list_memo_relations(&memo_id, Some(100), None)
                        .await
                }
            })
            .await?
            .relations;
        let shares = self
            .call(move |client| async move { client.memo_service_list_memo_shares(&memo_id).await })
            .await?
            .memo_shares;

        Ok(MemoDetailData {
            attachments,
            comments,
            reactions,
            relations,
            shares,
        })
    }

    pub async fn upload_memo_attachment(
        &self,
        memo_name: String,
        existing: Vec<types::Attachment>,
        path: PathBuf,
    ) -> Result<types::Attachment, ApiError> {
        let filename = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or(ApiError::Request("attachment has no valid filename".into()))?
            .to_string();
        let content = std::fs::read(&path).map_err(|error| ApiError::Request(error.to_string()))?;
        let attachment = types::Attachment {
            content: Some(BASE64.encode(content)),
            create_time: None,
            external_link: None,
            filename,
            media_metadata: None,
            memo: Some(memo_name.clone()),
            motion_media: None,
            name: None,
            size: None,
            type_: mime_guess::from_path(&path)
                .first_raw()
                .unwrap_or("application/octet-stream")
                .to_string(),
        };
        let created = self
            .call(move |client| async move {
                client
                    .attachment_service_create_attachment(None, &attachment)
                    .await
            })
            .await?;
        let mut attachments = existing;
        attachments.push(created.clone());
        let memo_id = resource_id(&memo_name).to_string();
        let request = types::SetMemoAttachmentsRequest {
            name: format!("memos/{memo_id}"),
            attachments,
        };
        self.call(move |client| async move {
            client
                .memo_service_set_memo_attachments(&memo_id, &request)
                .await
        })
        .await?;
        Ok(created)
    }

    pub async fn list_memo_views(
        &self,
        user_name: String,
    ) -> Result<Vec<types::MemoView>, ApiError> {
        let user_id = resource_id(&user_name).to_string();
        Ok(
            self.call(move |client| async move {
                client.memo_view_service_list_memo_views(&user_id).await
            })
            .await?
            .memo_views,
        )
    }

    pub async fn list_notifications(
        &self,
        user_name: String,
    ) -> Result<Vec<types::UserNotification>, ApiError> {
        let user_id = resource_id(&user_name).to_string();
        Ok(self
            .call(move |client| async move {
                client
                    .user_service_list_user_notifications(&user_id, None, Some(100), None)
                    .await
            })
            .await?
            .notifications)
    }

    pub async fn list_attachments(&self) -> Result<Vec<types::Attachment>, ApiError> {
        Ok(self
            .call(|client| async move {
                client
                    .attachment_service_list_attachments(
                        None,
                        Some("create_time desc"),
                        Some(100),
                        None,
                    )
                    .await
            })
            .await?
            .attachments)
    }

    pub async fn create_memo_comment(
        &self,
        memo_name: String,
        content: String,
    ) -> Result<types::Memo, ApiError> {
        let memo_id = resource_id(&memo_name).to_string();
        let comment = types::Memo {
            attachments: Vec::new(),
            content,
            create_time: None,
            creator: None,
            location: None,
            name: None,
            parent: Some(memo_name),
            pinned: Some(false),
            property: None,
            reactions: Vec::new(),
            relations: Vec::new(),
            snippet: None,
            state: types::MemoState::Normal,
            tags: Vec::new(),
            update_time: None,
            visibility: types::MemoVisibility::Private,
        };
        self.call(move |client| async move {
            client
                .memo_service_create_memo_comment(&memo_id, None, &comment)
                .await
        })
        .await
    }

    pub async fn upsert_memo_reaction(
        &self,
        memo_name: String,
        reaction_type: String,
    ) -> Result<types::Reaction, ApiError> {
        let memo_id = resource_id(&memo_name).to_string();
        let request = types::UpsertMemoReactionRequest {
            name: format!("memos/{memo_id}"),
            reaction: types::Reaction {
                content_id: format!("memos/{memo_id}"),
                create_time: None,
                creator: None,
                name: None,
                reaction_type,
            },
        };
        self.call(move |client| async move {
            client
                .memo_service_upsert_memo_reaction(&memo_id, &request)
                .await
        })
        .await
    }

    pub async fn create_memo_share(&self, memo_name: String) -> Result<types::MemoShare, ApiError> {
        let memo_id = resource_id(&memo_name).to_string();
        let share = types::MemoShare {
            create_time: None,
            expire_time: None,
            name: None,
        };
        self.call(move |client| async move {
            client
                .memo_service_create_memo_share(&memo_id, &share)
                .await
        })
        .await
    }

    pub async fn delete_memo_share(
        &self,
        memo_name: String,
        share_name: String,
    ) -> Result<(), ApiError> {
        let memo_id = resource_id(&memo_name).to_string();
        let share_id = resource_id(&share_name).to_string();
        self.call(move |client| async move {
            client
                .memo_service_delete_memo_share(&memo_id, &share_id)
                .await
        })
        .await
    }

    pub async fn sign_out(&self) -> Result<(), ApiError> {
        let result = self
            .call_without_refresh(|client| async move { client.auth_service_sign_out().await })
            .await;
        auth::clear_access_token(&self.client);
        *self
            .access_token_expires_at
            .write()
            .expect("access token expiry lock poisoned") = None;
        result
    }

    async fn call<T, F, Fut>(&self, operation: F) -> Result<T, ApiError>
    where
        T: Send + 'static,
        F: FnOnce(Client) -> Fut + Send + 'static,
        Fut: Future<Output = Result<ResponseValue<T>, ClientError<()>>> + Send + 'static,
    {
        let session = self.clone();
        let handle = self.runtime.spawn(async move {
            session.refresh_if_needed().await?;
            operation(session.client.clone())
                .await
                .map(ResponseValue::into_inner)
                .map_err(api_error)
        });
        handle
            .await
            .map_err(|error| ApiError::Runtime(error.to_string()))?
    }

    async fn call_without_refresh<T, F, Fut>(&self, operation: F) -> Result<T, ApiError>
    where
        T: Send + 'static,
        F: FnOnce(Client) -> Fut + Send + 'static,
        Fut: Future<Output = Result<ResponseValue<T>, ClientError<()>>> + Send + 'static,
    {
        let client = self.client.clone();
        let handle = self.runtime.spawn(async move {
            operation(client)
                .await
                .map(ResponseValue::into_inner)
                .map_err(api_error)
        });
        handle
            .await
            .map_err(|error| ApiError::Runtime(error.to_string()))?
    }

    async fn refresh_if_needed(&self) -> Result<(), ApiError> {
        let expires_at = *self
            .access_token_expires_at
            .read()
            .expect("access token expiry lock poisoned");
        let should_refresh = expires_at
            .map(|expires_at| expires_at <= Utc::now() + REFRESH_BUFFER)
            .unwrap_or(false);
        if !should_refresh {
            return Ok(());
        }

        let request = types::RefreshTokenRequest(serde_json::Map::new());
        let response = self
            .client
            .auth_service_refresh_token(&request)
            .await
            .map(ResponseValue::into_inner)
            .map_err(api_error)?;
        let token = response
            .access_token
            .ok_or(ApiError::MissingField("refreshed access token"))?;
        auth::set_access_token(&self.client, &token)
            .map_err(|error| ApiError::Request(error.to_string()))?;
        *self
            .access_token_expires_at
            .write()
            .expect("access token expiry lock poisoned") = response.expires_at;
        Ok(())
    }
}

fn resource_id(name: &str) -> &str {
    name.rsplit('/').next().unwrap_or(name)
}

fn api_error(error: ClientError<()>) -> ApiError {
    let status = error
        .status()
        .map(|status| format!("HTTP {status}: "))
        .unwrap_or_default();
    ApiError::Request(format!("{status}{error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_names_are_reduced_to_ids() {
        assert_eq!(resource_id("memos/abc-123"), "abc-123");
        assert_eq!(resource_id("abc-123"), "abc-123");
    }

    #[test]
    fn server_url_requires_http() {
        let runtime = Arc::new(Runtime::new().unwrap());
        assert!(matches!(
            ApiSession::new("memos.example.com", runtime),
            Err(ApiError::InvalidServerUrl(_))
        ));
    }
}
