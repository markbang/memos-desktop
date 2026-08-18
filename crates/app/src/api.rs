use std::{
    future::Future,
    path::{Path, PathBuf},
    sync::{
        Arc, RwLock,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration as StdDuration,
};

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use chrono::{DateTime, Duration, Utc};
use directories::ProjectDirs;
use futures::StreamExt as _;
use memos_api::{Client, Error as ClientError, ResponseValue, auth, types};
use serde::Deserialize;
use sha2::{Digest as _, Sha256};
use thiserror::Error;
use tokio::{runtime::Runtime, sync::Mutex};
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
    identity: Arc<()>,
    access_token_expires_at: Arc<RwLock<Option<DateTime<Utc>>>>,
    refresh_lock: Arc<Mutex<()>>,
}

#[derive(Default)]
pub struct MemoDetailData {
    pub attachments: Vec<types::Attachment>,
    pub comments: Vec<types::Memo>,
    pub reactions: Vec<types::Reaction>,
    pub relations: Vec<types::MemoRelation>,
    pub shares: Vec<types::MemoShare>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LiveEvent {
    #[serde(rename = "type")]
    pub kind: String,
    pub name: String,
    #[serde(default)]
    pub parent: Option<String>,
}

pub struct LiveSubscription {
    pub receiver: tokio::sync::mpsc::UnboundedReceiver<LiveEvent>,
    cancel: Arc<AtomicBool>,
}

impl LiveSubscription {
    pub fn into_parts(
        self,
    ) -> (
        tokio::sync::mpsc::UnboundedReceiver<LiveEvent>,
        Arc<AtomicBool>,
    ) {
        (self.receiver, self.cancel)
    }
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
            .user_agent(concat!("memos-desktop/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|error| ApiError::Request(error.to_string()))?;

        Ok(Self {
            client: Client::new_with_client(&base_url, http),
            base_url,
            runtime,
            identity: Arc::new(()),
            access_token_expires_at: Arc::new(RwLock::new(None)),
            refresh_lock: Arc::new(Mutex::new(())),
        })
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub(crate) fn same_session(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.identity, &other.identity)
    }

    pub async fn instance_profile(&self) -> Result<types::InstanceProfile, ApiError> {
        let profile = self
            .call_without_refresh(|client| async move {
                client.instance_service_get_instance_profile().await
            })
            .await?;
        if !is_supported_version(profile.version.as_deref()) {
            return Err(ApiError::Request(format!(
                "unsupported Memos version {}; this client targets 0.30.x",
                profile.version.as_deref().unwrap_or("unknown")
            )));
        }
        Ok(profile)
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

        let response = self.sign_in_request(request).await?;

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

    pub async fn sign_in_sso(
        &self,
        idp_name: String,
        code: String,
        redirect_uri: String,
        code_verifier: String,
    ) -> Result<types::User, ApiError> {
        let session = self.clone();
        let request = types::SignInRequest {
            password_credentials: None,
            sso_credentials: Some(types::SignInRequestSsoCredentials {
                code,
                code_verifier: Some(code_verifier),
                idp_name,
                redirect_uri,
            }),
        };
        let response = self.sign_in_request(request).await?;
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
            .execute(|client| async move { client.auth_service_get_current_user().await })
            .await?;
        response.user.ok_or(ApiError::MissingField("current user"))
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
        self.execute(
            move |client| async move { client.memo_service_create_memo(None, &memo).await },
        )
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
        self.execute(move |client| async move {
            client
                .memo_service_update_memo(&memo_id, Some(&update_mask), &memo)
                .await
        })
        .await
    }

    pub async fn delete_memo(&self, memo_name: String) -> Result<(), ApiError> {
        let memo_id = resource_id(&memo_name).to_string();
        self.execute(move |client| async move {
            client.memo_service_delete_memo(&memo_id, Some(false)).await
        })
        .await
    }

    pub async fn load_memo_detail(&self, memo_name: String) -> Result<MemoDetailData, ApiError> {
        let memo_id = resource_id(&memo_name).to_string();
        let mut attachments = Vec::new();
        let mut page_token = None;
        loop {
            let response = self
                .execute({
                    let memo_id = memo_id.clone();
                    let page_token = page_token.clone();
                    move |client| async move {
                        client
                            .memo_service_list_memo_attachments(
                                &memo_id,
                                Some(100),
                                page_token.as_deref(),
                            )
                            .await
                    }
                })
                .await?;
            attachments.extend(response.attachments);
            page_token = non_empty_token(response.next_page_token);
            if page_token.is_none() {
                break;
            }
        }

        let mut comments = Vec::new();
        let mut page_token = None;
        loop {
            let response = self
                .execute({
                    let memo_id = memo_id.clone();
                    let page_token = page_token.clone();
                    move |client| async move {
                        client
                            .memo_service_list_memo_comments(
                                &memo_id,
                                Some("create_time asc"),
                                Some(100),
                                page_token.as_deref(),
                            )
                            .await
                    }
                })
                .await?;
            comments.extend(response.memos);
            page_token = non_empty_token(response.next_page_token);
            if page_token.is_none() {
                break;
            }
        }

        let mut reactions = Vec::new();
        let mut page_token = None;
        loop {
            let response = self
                .execute({
                    let memo_id = memo_id.clone();
                    let page_token = page_token.clone();
                    move |client| async move {
                        client
                            .memo_service_list_memo_reactions(
                                &memo_id,
                                Some(100),
                                page_token.as_deref(),
                            )
                            .await
                    }
                })
                .await?;
            reactions.extend(response.reactions);
            page_token = non_empty_token(response.next_page_token);
            if page_token.is_none() {
                break;
            }
        }

        let mut relations = Vec::new();
        let mut page_token = None;
        loop {
            let response = self
                .execute({
                    let memo_id = memo_id.clone();
                    let page_token = page_token.clone();
                    move |client| async move {
                        client
                            .memo_service_list_memo_relations(
                                &memo_id,
                                Some(100),
                                page_token.as_deref(),
                            )
                            .await
                    }
                })
                .await?;
            relations.extend(response.relations);
            page_token = non_empty_token(response.next_page_token);
            if page_token.is_none() {
                break;
            }
        }
        let shares = self
            .execute(
                move |client| async move { client.memo_service_list_memo_shares(&memo_id).await },
            )
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
            .execute(move |client| async move {
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
        self.execute(move |client| async move {
            client
                .memo_service_set_memo_attachments(&memo_id, &request)
                .await
        })
        .await?;
        Ok(created)
    }

    pub async fn cache_attachment(
        &self,
        attachment: types::Attachment,
        thumbnail: bool,
    ) -> Result<PathBuf, ApiError> {
        if attachment
            .external_link
            .as_deref()
            .is_some_and(|link| !link.trim().is_empty())
        {
            return Err(ApiError::Request(
                "external attachments are opened at their source URL".into(),
            ));
        }
        let name = attachment
            .name
            .as_deref()
            .ok_or(ApiError::MissingField("attachment name"))?;
        let filename = Path::new(&attachment.filename)
            .file_name()
            .and_then(|filename| filename.to_str())
            .ok_or_else(|| ApiError::Request("attachment has no valid filename".into()))?;
        if filename != attachment.filename {
            return Err(ApiError::Request(
                "attachment filename must not contain path separators".into(),
            ));
        }
        let mut url = Url::parse(&format!("{}/", self.base_url.trim_end_matches('/')))
            .map_err(|error| ApiError::Request(error.to_string()))?;
        {
            let mut segments = url
                .path_segments_mut()
                .map_err(|_| ApiError::Request("Memos URL cannot contain path segments".into()))?;
            segments.pop_if_empty().push("file");
            for segment in name.split('/') {
                segments.push(segment);
            }
            segments.push(filename);
        }
        if thumbnail {
            url.query_pairs_mut().append_pair("thumbnail", "true");
        }

        let project_dirs =
            ProjectDirs::from("com", "Memos Desktop", "Memos Desktop").ok_or_else(|| {
                ApiError::Request("application cache directory is unavailable".into())
            })?;
        let server_hash = Sha256::digest(self.base_url.as_bytes())[..8]
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        let attachment_id = resource_id(name);
        let cache_dir = project_dirs
            .cache_dir()
            .join("attachments")
            .join(server_hash)
            .join(attachment_id);
        let cached_filename = if thumbnail {
            format!("thumbnail-{filename}")
        } else {
            filename.to_string()
        };
        let cache_path = cache_dir.join(cached_filename);
        if cache_path.is_file() {
            return Ok(cache_path);
        }

        let session = self.clone();
        let handle = self.runtime.spawn(async move {
            session.refresh_if_needed().await?;
            let mut request = session.client.http_client().get(url.clone());
            if let Some(header) = auth::authorization_header(&session.client) {
                request = request.header(reqwest::header::AUTHORIZATION, header);
            }
            let response = request
                .send()
                .await
                .map_err(|error| ApiError::Request(error.to_string()))?;
            let status = response.status();
            if !status.is_success() {
                let body = response.text().await.unwrap_or_default();
                let body = body.chars().take(512).collect::<String>();
                return Err(ApiError::Request(format!(
                    "HTTP {status} from {url}: {body}"
                )));
            }
            let bytes = response
                .bytes()
                .await
                .map_err(|error| ApiError::Request(error.to_string()))?;
            std::fs::create_dir_all(&cache_dir)
                .map_err(|error| ApiError::Request(error.to_string()))?;
            let temporary =
                cache_path.with_extension(format!("download-{}", rand::random::<u64>()));
            std::fs::write(&temporary, bytes)
                .map_err(|error| ApiError::Request(error.to_string()))?;
            if let Err(error) = std::fs::rename(&temporary, &cache_path) {
                let another_download_completed = cache_path.is_file();
                _ = std::fs::remove_file(&temporary);
                if !another_download_completed {
                    return Err(ApiError::Request(error.to_string()));
                }
            }
            Ok(cache_path)
        });
        handle
            .await
            .map_err(|error| ApiError::Runtime(error.to_string()))?
    }

    pub async fn list_memo_views(
        &self,
        user_name: String,
    ) -> Result<Vec<types::Shortcut>, ApiError> {
        let user_id = resource_id(&user_name).to_string();
        Ok(self
            .execute(
                move |client| async move { client.shortcut_service_list_shortcuts(&user_id).await },
            )
            .await?
            .shortcuts)
    }

    pub async fn list_notifications_page(
        &self,
        user_name: String,
        page_token: Option<String>,
    ) -> Result<types::ListUserNotificationsResponse, ApiError> {
        let user_id = resource_id(&user_name).to_string();
        self.execute(move |client| async move {
            client
                .user_service_list_user_notifications(
                    &user_id,
                    None,
                    Some(100),
                    page_token.as_deref(),
                )
                .await
        })
        .await
    }

    pub async fn list_attachments_page(
        &self,
        page_token: Option<String>,
    ) -> Result<types::ListAttachmentsResponse, ApiError> {
        self.execute(move |client| async move {
            client
                .attachment_service_list_attachments(
                    None,
                    Some("create_time desc"),
                    Some(100),
                    page_token.as_deref(),
                )
                .await
        })
        .await
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
        self.execute(move |client| async move {
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
        self.execute(move |client| async move {
            client
                .memo_service_upsert_memo_reaction(&memo_id, &request)
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
        self.execute(move |client| async move {
            client
                .memo_service_delete_memo_share(&memo_id, &share_id)
                .await
        })
        .await
    }

    pub fn subscribe_live(&self) -> LiveSubscription {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        let cancel = Arc::new(AtomicBool::new(false));
        let worker_cancel = cancel.clone();
        let session = self.clone();
        self.runtime.spawn(async move {
            while !worker_cancel.load(Ordering::Acquire) && !sender.is_closed() {
                if session.refresh_if_needed().await.is_err() {
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    continue;
                }
                let url = format!("{}/api/v1/sse", session.client.base_url());
                let mut request = session
                    .client
                    .http_client()
                    .get(url)
                    .header(reqwest::header::ACCEPT, "text/event-stream");
                if let Some(header) = memos_api::auth::authorization_header(&session.client) {
                    request = request.header(reqwest::header::AUTHORIZATION, header);
                }
                let response = match request.send().await {
                    Ok(response) if response.status().is_success() => response,
                    _ => {
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                        continue;
                    }
                };
                let mut stream = response.bytes_stream();
                let mut buffer = Vec::new();
                while !worker_cancel.load(Ordering::Acquire) {
                    tokio::select! {
                        chunk = stream.next() => match chunk {
                            Some(Ok(chunk)) => {
                                buffer.extend_from_slice(&chunk);
                                for event in drain_sse_events(&mut buffer) {
                                    if sender.send(event).is_err() {
                                        worker_cancel.store(true, Ordering::Release);
                                        break;
                                    }
                                }
                                if buffer.len() > 1024 * 1024 {
                                    buffer.clear();
                                }
                            }
                            Some(Err(_)) | None => break,
                        },
                        _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => {}
                    }
                }
                if !worker_cancel.load(Ordering::Acquire) && !sender.is_closed() {
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                }
            }
        });
        LiveSubscription { receiver, cancel }
    }

    pub async fn sign_out(&self) -> Result<(), ApiError> {
        let session = self.clone();
        let result = self
            .runtime
            .spawn(async move {
                let url = format!("{}/memos.api.v1.AuthService/SignOut", session.base_url);
                let mut request = session
                    .client
                    .http_client()
                    .post(&url)
                    .header(reqwest::header::CONTENT_TYPE, "application/json")
                    .header("connect-protocol-version", "1")
                    .body("{}");
                if let Some(header) = auth::authorization_header(&session.client) {
                    request = request.header(reqwest::header::AUTHORIZATION, header);
                }
                if let Some(cookie) = auth::refresh_cookie_header(&session.client) {
                    request = request.header(reqwest::header::COOKIE, cookie);
                }
                let response = request
                    .send()
                    .await
                    .map_err(|error| ApiError::Request(error.to_string()))?;
                response_error(response, &url).await
            })
            .await
            .map_err(|error| ApiError::Runtime(error.to_string()))?;
        auth::clear_access_token(&self.client);
        *self
            .access_token_expires_at
            .write()
            .expect("access token expiry lock poisoned") = None;
        result
    }

    pub(crate) async fn execute<T, F, Fut>(&self, operation: F) -> Result<T, ApiError>
    where
        T: Send + 'static,
        F: FnOnce(Client) -> Fut + Send + 'static,
        Fut: Future<Output = Result<ResponseValue<T>, ClientError<()>>> + Send + 'static,
    {
        let session = self.clone();
        let handle = self.runtime.spawn(async move {
            session.refresh_if_needed().await?;
            match operation(session.client.clone()).await {
                Ok(response) => Ok(response.into_inner()),
                Err(error) => Err(api_error(error).await),
            }
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
            match operation(client).await {
                Ok(response) => Ok(response.into_inner()),
                Err(error) => Err(api_error(error).await),
            }
        });
        handle
            .await
            .map_err(|error| ApiError::Runtime(error.to_string()))?
    }

    async fn sign_in_request(
        &self,
        request: types::SignInRequest,
    ) -> Result<types::SignInResponse, ApiError> {
        let session = self.clone();
        let handle = self.runtime.spawn(async move {
            match session.client.auth_service_sign_in(&request).await {
                Ok(response) => {
                    capture_refresh_cookie(&session.client, response.headers())?;
                    Ok(response.into_inner())
                }
                Err(error) => Err(api_error(error).await),
            }
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

        self.refresh_access_token().await
    }

    async fn refresh_access_token(&self) -> Result<(), ApiError> {
        let _guard = self.refresh_lock.lock().await;
        let expires_at = *self
            .access_token_expires_at
            .read()
            .expect("access token expiry lock poisoned");
        if expires_at
            .map(|expires_at| expires_at > Utc::now() + REFRESH_BUFFER)
            .unwrap_or(false)
        {
            return Ok(());
        }
        self.perform_refresh().await
    }

    async fn perform_refresh(&self) -> Result<(), ApiError> {
        let url = format!("{}/memos.api.v1.AuthService/RefreshToken", self.base_url);
        let mut request = self
            .client
            .http_client()
            .post(&url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header("connect-protocol-version", "1")
            .body("{}");
        if let Some(cookie) = auth::refresh_cookie_header(&self.client) {
            request = request.header(reqwest::header::COOKIE, cookie);
        }
        let response = request
            .send()
            .await
            .map_err(|error| ApiError::Request(error.to_string()))?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            let body = body.chars().take(512).collect::<String>();
            return Err(ApiError::Request(format!(
                "HTTP {status} from {url}: {body}"
            )));
        }
        let headers = response.headers().clone();
        let response = response
            .json::<types::RefreshTokenResponse>()
            .await
            .map_err(|error| ApiError::Request(error.to_string()))?;
        capture_refresh_cookie(&self.client, &headers)?;
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

    #[cfg(test)]
    async fn refresh_access_token_for_test(&self) -> Result<(), ApiError> {
        let _guard = self.refresh_lock.lock().await;
        self.perform_refresh().await
    }
}

fn drain_sse_events(buffer: &mut Vec<u8>) -> Vec<LiveEvent> {
    let mut events = Vec::new();
    while let Some(position) = buffer.windows(2).position(|window| window == b"\n\n") {
        let frame = buffer.drain(..position + 2).collect::<Vec<_>>();
        for line in frame.split(|byte| *byte == b'\n') {
            if let Some(data) = line.strip_prefix(b"data: ")
                && let Ok(event) = serde_json::from_slice::<LiveEvent>(data)
            {
                events.push(event);
            }
        }
    }
    events
}

pub(crate) fn resource_id(name: &str) -> &str {
    name.rsplit('/').next().unwrap_or(name)
}

fn is_supported_version(version: Option<&str>) -> bool {
    let mut parts = version.unwrap_or_default().split('.');
    parts.next() == Some("0") && parts.next() == Some("30")
}

fn non_empty_token(token: Option<String>) -> Option<String> {
    token.filter(|token| !token.is_empty())
}

fn capture_refresh_cookie(
    client: &Client,
    headers: &reqwest::header::HeaderMap,
) -> Result<(), ApiError> {
    let cookie = ["grpc-metadata-set-cookie", "set-cookie"]
        .into_iter()
        .flat_map(|name| headers.get_all(name).iter())
        .filter_map(|value| value.to_str().ok())
        .filter_map(|value| value.split(';').next())
        .find(|value| value.starts_with("memos_refresh="));
    if let Some(cookie) = cookie {
        auth::set_refresh_cookie(client, cookie)
            .map_err(|error| ApiError::Request(format!("invalid refresh cookie: {error}")))?;
    }
    Ok(())
}

async fn api_error(error: ClientError<()>) -> ApiError {
    if let ClientError::UnexpectedResponse(response) = error {
        let status = response.status();
        let url = response.url().clone();
        let body = response
            .text()
            .await
            .unwrap_or_else(|body_error| format!("unable to read response body: {body_error}"));
        let body = body.chars().take(512).collect::<String>();
        return ApiError::Request(format!("HTTP {status} from {url}: {body}"));
    }
    let status = error
        .status()
        .map(|status| format!("HTTP {status}: "))
        .unwrap_or_default();
    ApiError::Request(format!("{status}{error}"))
}

async fn response_error(response: reqwest::Response, url: &str) -> Result<(), ApiError> {
    let status = response.status();
    if status.is_success() {
        return Ok(());
    }
    let body = response.text().await.unwrap_or_default();
    let body = body.chars().take(512).collect::<String>();
    Err(ApiError::Request(format!(
        "HTTP {status} from {url}: {body}"
    )))
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
    fn sse_frames_are_buffered_and_decoded() {
        let mut buffer = b": connected\n\ndata: {\"type\":\"memo.updated\",\"name\":\"memos/1\"}\n\ndata: {\"type\":\"memo".to_vec();
        let events = drain_sse_events(&mut buffer);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].kind, "memo.updated");
        assert_eq!(events[0].name, "memos/1");
        buffer.extend_from_slice(b".deleted\",\"name\":\"memos/2\"}\n\n");
        let events = drain_sse_events(&mut buffer);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].kind, "memo.deleted");
    }

    #[test]
    fn supported_version_accepts_v030_patches_only() {
        assert!(is_supported_version(Some("0.30.0")));
        assert!(is_supported_version(Some("0.30.7-dev")));
        assert!(!is_supported_version(Some("0.31.0")));
        assert!(!is_supported_version(None));
    }

    #[test]
    fn refresh_cookie_is_captured_from_gateway_metadata() {
        let client = Client::new("https://memos.example.com");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "grpc-metadata-set-cookie",
            reqwest::header::HeaderValue::from_static(
                "memos_refresh=refresh-token; Path=/; HttpOnly; SameSite=Lax",
            ),
        );

        capture_refresh_cookie(&client, &headers).unwrap();

        assert_eq!(
            memos_api::auth::refresh_cookie_header(&client).unwrap(),
            "memos_refresh=refresh-token"
        );
    }

    #[test]
    fn server_url_requires_http() {
        let runtime = Arc::new(Runtime::new().unwrap());
        assert!(matches!(
            ApiSession::new("memos.example.com", runtime),
            Err(ApiError::InvalidServerUrl(_))
        ));
    }

    #[test]
    #[ignore = "requires a disposable Memos v0.30 instance"]
    fn live_v030_core_round_trip() {
        let server_url = std::env::var("MEMOS_LIVE_URL")
            .expect("set MEMOS_LIVE_URL to a disposable Memos instance");
        let username = std::env::var("MEMOS_LIVE_USERNAME")
            .expect("set MEMOS_LIVE_USERNAME to an administrator username");
        let password = std::env::var("MEMOS_LIVE_PASSWORD")
            .expect("set MEMOS_LIVE_PASSWORD to the administrator password");
        let runtime = Arc::new(Runtime::new().unwrap());
        let session = ApiSession::new(&server_url, runtime.clone()).unwrap();

        runtime.block_on(async {
            let profile = session.instance_profile().await.unwrap();
            assert!(
                profile
                    .version
                    .as_deref()
                    .is_some_and(|version| version.starts_with("0.30"))
            );
            let user = session.sign_in_password(username, password).await.unwrap();
            let user_name = user.name.clone().unwrap();
            assert!(
                memos_api::auth::refresh_cookie_header(&session.client).is_some(),
                "sign-in response did not expose the refresh cookie"
            );
            session.refresh_access_token_for_test().await.unwrap();
            assert_eq!(session.current_user().await.unwrap().name, user.name);

            let suffix = Utc::now().timestamp_millis();
            let registration_username = format!("desktop-live-{suffix}");
            let registration_password = format!("live-password-{suffix}");
            let registration_session = ApiSession::new(&server_url, runtime.clone()).unwrap();
            let registered = registration_session
                .create_user(types::User {
                    avatar_url: None,
                    create_time: None,
                    description: None,
                    display_name: Some("Desktop live user".into()),
                    email: None,
                    name: None,
                    password: Some(registration_password.clone()),
                    role: types::UserRole::User,
                    state: types::UserState::Normal,
                    update_time: None,
                    username: registration_username.clone(),
                })
                .await
                .unwrap();
            let registered_name = registered.name.clone().unwrap();
            let mut registered_user = registration_session
                .sign_in_password(registration_username, registration_password)
                .await
                .unwrap();
            assert_eq!(registered_user.name, registered.name);
            registered_user.display_name = Some("Updated desktop live user".into());
            registered_user.password = None;
            registered_user = registration_session
                .update_user(registered_user, "display_name".into())
                .await
                .unwrap();
            assert_eq!(
                registered_user.display_name.as_deref(),
                Some("Updated desktop live user")
            );

            let (mut live_events, live_cancel) = session.subscribe_live().into_parts();
            tokio::time::sleep(StdDuration::from_millis(100)).await;
            let first = session
                .create_memo(
                    format!("# Live API test {suffix}\n\n- [ ] round trip"),
                    types::MemoVisibility::Private,
                )
                .await
                .unwrap();
            let first_name = first.name.clone().unwrap();
            let live_event = tokio::time::timeout(StdDuration::from_secs(3), async {
                while let Some(event) = live_events.recv().await {
                    if event.name == first_name {
                        return Some(event);
                    }
                }
                None
            })
            .await
            .unwrap()
            .unwrap();
            assert_eq!(live_event.kind, "memo.created");
            live_cancel.store(true, Ordering::Release);

            let second = session
                .create_memo(
                    format!("Related live API test {suffix}"),
                    types::MemoVisibility::Private,
                )
                .await
                .unwrap();
            let second_name = second.name.clone().unwrap();

            let listed = session
                .list_memos_page(
                    Some(format!("creator == {user_name:?}")),
                    Some("create_time desc".into()),
                    10,
                    None,
                    false,
                )
                .await
                .unwrap();
            assert!(
                listed
                    .memos
                    .iter()
                    .any(|memo| memo.name.as_ref() == Some(&first_name))
            );
            assert_eq!(
                session.get_memo(first_name.clone()).await.unwrap().name,
                first.name
            );

            let mut updated = first.clone();
            updated.content.push_str("\n\nupdated");
            updated.visibility = types::MemoVisibility::Protected;
            updated.pinned = Some(true);
            let updated = session
                .update_memo(updated, "content,visibility,pinned".into())
                .await
                .unwrap();
            assert!(updated.content.ends_with("updated"));
            let mut updated = updated;
            updated.location = Some(types::Location {
                latitude: Some(37.7749),
                longitude: Some(-122.4194),
                placeholder: Some("Live test location".into()),
            });
            let mut updated = session
                .update_memo(updated, "location".into())
                .await
                .unwrap();
            assert_eq!(
                updated
                    .location
                    .as_ref()
                    .and_then(|location| location.placeholder.as_deref()),
                Some("Live test location")
            );
            updated.state = types::MemoState::Archived;
            let mut updated = session
                .update_memo(updated, "state".into())
                .await
                .unwrap();
            assert_eq!(updated.state, types::MemoState::Archived);
            updated.state = types::MemoState::Normal;
            session.update_memo(updated, "state".into()).await.unwrap();

            let comment = registration_session
                .create_memo_comment(first_name.clone(), "integration comment".into())
                .await
                .unwrap();
            let mut notification = None;
            for _ in 0..20 {
                let notifications = session
                    .list_notifications_page(user_name.clone(), None)
                    .await
                    .unwrap();
                notification = notifications.notifications.into_iter().find(|item| {
                    item.memo_comment
                        .as_ref()
                        .and_then(|payload| payload.memo.as_ref())
                        == comment.name.as_ref()
                });
                if notification.is_some() {
                    break;
                }
                tokio::time::sleep(StdDuration::from_millis(50)).await;
            }
            let mut notification =
                notification.unwrap_or_else(|| panic!("comment notification was not created"));
            let notification_name = notification.name.clone().unwrap();
            notification.status = Some(types::UserNotificationStatus::Archived);
            assert_eq!(
                session
                    .update_notification(notification)
                    .await
                    .unwrap()
                    .status,
                Some(types::UserNotificationStatus::Archived)
            );
            session
                .delete_notification(notification_name)
                .await
                .unwrap();
            let reaction = session
                .upsert_memo_reaction(first_name.clone(), "+1".into())
                .await
                .unwrap();
            session
                .set_memo_relations(
                    first_name.clone(),
                    vec![types::MemoRelation {
                        memo: types::MemoRelationMemo {
                            name: first_name.clone(),
                            snippet: None,
                        },
                        related_memo: types::MemoRelationMemo {
                            name: second_name.clone(),
                            snippet: None,
                        },
                        type_: types::MemoRelationType::Reference,
                    }],
                )
                .await
                .unwrap();
            let share = session
                .create_memo_share_with_expiry(
                    first_name.clone(),
                    Some(Utc::now() + Duration::days(1)),
                )
                .await
                .unwrap();

            let attachment_path = std::env::temp_dir().join(format!("memos-live-{suffix}.txt"));
            std::fs::write(&attachment_path, b"memos desktop live test").unwrap();
            let mut attachment = session
                .upload_memo_attachment(first_name.clone(), Vec::new(), attachment_path.clone())
                .await
                .unwrap();
            assert_eq!(
                session
                    .get_attachment(attachment.name.clone().unwrap())
                    .await
                    .unwrap()
                    .filename,
                attachment.filename
            );
            attachment.filename = format!("renamed-{suffix}.txt");
            attachment = session
                .update_attachment(attachment, "filename".into())
                .await
                .unwrap();
            let cached_attachment = session
                .cache_attachment(attachment.clone(), false)
                .await
                .unwrap();
            assert_eq!(
                std::fs::read(cached_attachment).unwrap(),
                b"memos desktop live test"
            );
            let image_path = std::env::temp_dir().join(format!("memos-live-{suffix}.png"));
            std::fs::write(
                &image_path,
                BASE64
                    .decode("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+A8AAQUBAScY42YAAAAASUVORK5CYII=")
                    .unwrap(),
            )
            .unwrap();
            let image_attachment = session
                .upload_memo_attachment(
                    first_name.clone(),
                    vec![attachment.clone()],
                    image_path.clone(),
                )
                .await
                .unwrap();
            assert!(
                session
                    .cache_attachment(image_attachment.clone(), true)
                    .await
                    .unwrap()
                    .is_file()
            );
            let detail = session.load_memo_detail(first_name.clone()).await.unwrap();
            assert_eq!(detail.comments.len(), 1);
            assert_eq!(detail.reactions.len(), 1);
            assert!(
                detail
                    .relations
                    .iter()
                    .any(|relation| relation.type_ == types::MemoRelationType::Reference)
            );
            assert_eq!(detail.shares.len(), 1);
            assert_eq!(detail.attachments.len(), 2);

            let mut shortcut = session
                .create_memo_view(
                    user_name.clone(),
                    types::Shortcut {
                        filter: Some("has_task_list == true".into()),
                        name: None,
                        title: format!("Live test {suffix}"),
                    },
                )
                .await
                .unwrap();
            shortcut.title = format!("Updated live test {suffix}");
            shortcut = session
                .update_memo_view(shortcut, "title,filter".into())
                .await
                .unwrap();
            assert!(shortcut.title.starts_with("Updated"));
            assert!(
                session
                    .list_memo_views(user_name.clone())
                    .await
                    .unwrap()
                    .iter()
                    .any(|item| item.name == shortcut.name)
            );

            let token = session
                .create_access_token(
                    user_name.clone(),
                    types::CreatePersonalAccessTokenRequest {
                        description: Some(format!("live test {suffix}")),
                        expires_in_days: Some(1),
                        parent: user_name.clone(),
                    },
                )
                .await
                .unwrap();
            assert!(token.token.is_some());
            let token_name = token.personal_access_token.unwrap().name.unwrap();

            let mut webhook = session
                .create_webhook(
                    user_name.clone(),
                    types::UserWebhook {
                        create_time: None,
                        display_name: Some(format!("Live test {suffix}")),
                        name: None,
                        signing_secret: Some(format!("secret-{suffix}")),
                        signing_secret_set: None,
                        update_time: None,
                        url: Some("https://example.com/memos-live-test".into()),
                    },
                )
                .await
                .unwrap();
            webhook.display_name = Some(format!("Updated live test {suffix}"));
            webhook.signing_secret = None;
            webhook = session.update_webhook(webhook).await.unwrap();
            assert!(webhook
                .display_name
                .as_deref()
                .is_some_and(|name| name.starts_with("Updated")));
            assert!(
                !session
                    .get_webhook_secret(webhook.name.clone().unwrap())
                    .await
                    .unwrap()
                    .is_empty()
            );

            let user_settings = session
                .list_user_settings(user_name.clone())
                .await
                .unwrap();
            assert!(!user_settings.is_empty());
            session
                .update_user_setting(user_settings[0].clone(), None)
                .await
                .unwrap();
            session.get_user_stats(user_name.clone()).await.unwrap();
            session.list_all_user_stats().await.unwrap();
            session.get_instance_stats().await.unwrap();
            let instance_settings = session
                .list_instance_settings(vec!["instance/settings/GENERAL".into()])
                .await
                .unwrap();
            session
                .update_instance_setting(instance_settings[0].clone(), None)
                .await
                .unwrap();
            session.list_identity_providers().await.unwrap();
            session.list_users_page(None, None, false).await.unwrap();

            session.delete_webhook(webhook.name.unwrap()).await.unwrap();
            session.delete_access_token(token_name).await.unwrap();
            session
                .delete_memo_view(shortcut.name.unwrap())
                .await
                .unwrap();
            session
                .delete_attachment(attachment.name.unwrap())
                .await
                .unwrap();
            session
                .delete_attachment(image_attachment.name.unwrap())
                .await
                .unwrap();
            session
                .delete_memo_reaction(reaction.name.unwrap())
                .await
                .unwrap();
            session
                .delete_memo_share(first_name.clone(), share.name.unwrap())
                .await
                .unwrap();
            session.delete_memo(comment.name.unwrap()).await.unwrap();
            session.delete_memo(second_name).await.unwrap();
            session.delete_memo(first_name).await.unwrap();
            registration_session.sign_out().await.unwrap();
            session.delete_user(registered_name, true).await.unwrap();
            session.sign_out().await.unwrap();
            std::fs::remove_file(attachment_path).unwrap();
            std::fs::remove_file(image_path).unwrap();
        });
    }
}
