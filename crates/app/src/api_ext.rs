use std::path::PathBuf;

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use memos_api::types;

use crate::api::{ApiError, ApiSession, resource_id};

impl ApiSession {
    pub async fn list_memos_page(
        &self,
        filter: Option<String>,
        order_by: Option<String>,
        page_size: i32,
        page_token: Option<String>,
        archived: bool,
    ) -> Result<types::ListMemosResponse, ApiError> {
        let state = if archived {
            types::MemoServiceListMemosState::Archived
        } else {
            types::MemoServiceListMemosState::Normal
        };
        self.execute(move |client| async move {
            client
                .memo_service_list_memos(
                    filter.as_deref(),
                    order_by.as_deref(),
                    Some(page_size),
                    page_token.as_deref(),
                    Some(false),
                    Some(state),
                )
                .await
        })
        .await
    }

    pub async fn get_memo(&self, memo_name: String) -> Result<types::Memo, ApiError> {
        let memo_id = resource_id(&memo_name).to_string();
        self.execute(move |client| async move { client.memo_service_get_memo(&memo_id).await })
            .await
    }

    pub async fn get_shared_memo(&self, share_token: String) -> Result<types::Memo, ApiError> {
        self.execute(move |client| async move {
            client.memo_service_get_shared_memo(&share_token).await
        })
        .await
    }

    pub async fn batch_get_link_metadata(
        &self,
        urls: Vec<String>,
    ) -> Result<Vec<types::LinkMetadata>, ApiError> {
        let request = types::BatchGetLinkMetadataRequest { urls };
        Ok(self
            .execute(move |client| async move {
                client.memo_service_batch_get_link_metadata(&request).await
            })
            .await?
            .link_metadata)
    }

    pub async fn set_memo_relations(
        &self,
        memo_name: String,
        relations: Vec<types::MemoRelation>,
    ) -> Result<(), ApiError> {
        let memo_id = resource_id(&memo_name).to_string();
        let request = types::SetMemoRelationsRequest {
            name: memo_name,
            relations,
        };
        self.execute(move |client| async move {
            client
                .memo_service_set_memo_relations(&memo_id, &request)
                .await
        })
        .await
    }

    pub async fn delete_memo_reaction(&self, reaction_name: String) -> Result<(), ApiError> {
        let (memo_id, reaction_id) =
            nested_resource_ids(&reaction_name, "memos", "reactions", "reaction")?;
        self.execute(move |client| async move {
            client
                .memo_service_delete_memo_reaction(&memo_id, &reaction_id)
                .await
        })
        .await
    }

    pub async fn set_memo_attachments(
        &self,
        memo_name: String,
        attachments: Vec<types::Attachment>,
    ) -> Result<(), ApiError> {
        let memo_id = resource_id(&memo_name).to_string();
        let request = types::SetMemoAttachmentsRequest {
            name: memo_name,
            attachments,
        };
        self.execute(move |client| async move {
            client
                .memo_service_set_memo_attachments(&memo_id, &request)
                .await
        })
        .await
    }

    #[cfg(test)]
    pub async fn get_attachment(
        &self,
        attachment_name: String,
    ) -> Result<types::Attachment, ApiError> {
        let id = resource_id(&attachment_name).to_string();
        self.execute(
            move |client| async move { client.attachment_service_get_attachment(&id).await },
        )
        .await
    }

    pub async fn update_attachment(
        &self,
        attachment: types::Attachment,
        update_mask: String,
    ) -> Result<types::Attachment, ApiError> {
        let id = resource_id(
            attachment
                .name
                .as_deref()
                .ok_or(ApiError::MissingField("attachment name"))?,
        )
        .to_string();
        self.execute(move |client| async move {
            client
                .attachment_service_update_attachment(&id, Some(&update_mask), &attachment)
                .await
        })
        .await
    }

    pub async fn delete_attachment(&self, attachment_name: String) -> Result<(), ApiError> {
        let id = resource_id(&attachment_name).to_string();
        self.execute(
            move |client| async move { client.attachment_service_delete_attachment(&id).await },
        )
        .await
    }

    pub async fn upload_attachment_file(
        &self,
        path: PathBuf,
    ) -> Result<types::Attachment, ApiError> {
        let filename = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| ApiError::Request("attachment has no valid filename".into()))?
            .to_string();
        let content = std::fs::read(&path).map_err(|error| ApiError::Request(error.to_string()))?;
        let attachment = types::Attachment {
            content: Some(BASE64.encode(content)),
            create_time: None,
            external_link: None,
            filename,
            memo: None,
            motion_media: None,
            name: None,
            size: None,
            type_: mime_guess::from_path(&path)
                .first_raw()
                .unwrap_or("application/octet-stream")
                .to_string(),
        };
        self.execute(move |client| async move {
            client
                .attachment_service_create_attachment(None, &attachment)
                .await
        })
        .await
    }

    pub async fn create_external_attachment(
        &self,
        filename: String,
        external_link: String,
        mime_type: String,
    ) -> Result<types::Attachment, ApiError> {
        let attachment = types::Attachment {
            content: None,
            create_time: None,
            external_link: Some(external_link),
            filename,
            memo: None,
            motion_media: None,
            name: None,
            size: None,
            type_: mime_type,
        };
        self.execute(move |client| async move {
            client
                .attachment_service_create_attachment(None, &attachment)
                .await
        })
        .await
    }

    pub async fn transcribe_audio(&self, path: PathBuf) -> Result<String, ApiError> {
        let filename = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| ApiError::Request("audio file has no valid filename".into()))?
            .to_string();
        let content = std::fs::read(&path).map_err(|error| ApiError::Request(error.to_string()))?;
        let request = types::TranscribeRequest {
            audio: types::TranscriptionAudio {
                content: Some(BASE64.encode(content)),
                content_type: Some(
                    mime_guess::from_path(&path)
                        .first_raw()
                        .unwrap_or("application/octet-stream")
                        .to_string(),
                ),
                filename: Some(filename),
                uri: None,
            },
        };
        Ok(self
            .execute(move |client| async move { client.ai_service_transcribe(&request).await })
            .await?
            .text
            .unwrap_or_default())
    }

    pub async fn create_memo_share_with_expiry(
        &self,
        memo_name: String,
        expire_time: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<types::MemoShare, ApiError> {
        let memo_id = resource_id(&memo_name).to_string();
        let share = types::MemoShare {
            create_time: None,
            expire_time,
            name: None,
        };
        self.execute(move |client| async move {
            client
                .memo_service_create_memo_share(&memo_id, &share)
                .await
        })
        .await
    }

    pub async fn create_memo_view(
        &self,
        user_name: String,
        view: types::Shortcut,
    ) -> Result<types::Shortcut, ApiError> {
        let user_id = resource_id(&user_name).to_string();
        self.execute(move |client| async move {
            client
                .shortcut_service_create_shortcut(&user_id, Some(false), &view)
                .await
        })
        .await
    }

    pub async fn update_memo_view(
        &self,
        view: types::Shortcut,
        update_mask: String,
    ) -> Result<types::Shortcut, ApiError> {
        let name = view
            .name
            .as_deref()
            .ok_or(ApiError::MissingField("memo view name"))?;
        let (user_id, view_id) = nested_resource_ids(name, "users", "shortcuts", "shortcut")?;
        self.execute(move |client| async move {
            client
                .shortcut_service_update_shortcut(&user_id, &view_id, Some(&update_mask), &view)
                .await
        })
        .await
    }

    pub async fn delete_memo_view(&self, view_name: String) -> Result<(), ApiError> {
        let (user_id, view_id) = nested_resource_ids(&view_name, "users", "shortcuts", "shortcut")?;
        self.execute(move |client| async move {
            client
                .shortcut_service_delete_shortcut(&user_id, &view_id)
                .await
        })
        .await
    }

    pub async fn update_notification(
        &self,
        notification: types::UserNotification,
    ) -> Result<types::UserNotification, ApiError> {
        let name = notification
            .name
            .as_deref()
            .ok_or(ApiError::MissingField("notification name"))?;
        let (user_id, notification_id) =
            nested_resource_ids(name, "users", "notifications", "notification")?;
        self.execute(move |client| async move {
            client
                .user_service_update_user_notification(
                    &user_id,
                    &notification_id,
                    Some("status"),
                    &notification,
                )
                .await
        })
        .await
    }

    pub async fn delete_notification(&self, notification_name: String) -> Result<(), ApiError> {
        let (user_id, notification_id) =
            nested_resource_ids(&notification_name, "users", "notifications", "notification")?;
        self.execute(move |client| async move {
            client
                .user_service_delete_user_notification(&user_id, &notification_id)
                .await
        })
        .await
    }

    pub async fn list_users_page(
        &self,
        filter: Option<String>,
        page_token: Option<String>,
        show_deleted: bool,
    ) -> Result<types::ListUsersResponse, ApiError> {
        self.execute(move |client| async move {
            client
                .user_service_list_users(
                    filter.as_deref(),
                    Some(100),
                    page_token.as_deref(),
                    Some(show_deleted),
                )
                .await
        })
        .await
    }

    pub async fn list_all_users(&self, show_deleted: bool) -> Result<Vec<types::User>, ApiError> {
        let mut users = Vec::new();
        let mut page_token = None;
        loop {
            let response = self.list_users_page(None, page_token, show_deleted).await?;
            users.extend(response.users);
            page_token = response.next_page_token.filter(|token| !token.is_empty());
            if page_token.is_none() {
                return Ok(users);
            }
        }
    }

    pub async fn get_user(&self, user_name: String) -> Result<types::User, ApiError> {
        let user_id = resource_id(&user_name).to_string();
        self.execute(
            move |client| async move { client.user_service_get_user(&user_id, None).await },
        )
        .await
    }

    pub async fn create_user(&self, user: types::User) -> Result<types::User, ApiError> {
        let user_id = user.username.clone();
        self.execute(move |client| async move {
            client
                .user_service_create_user(None, Some(&user_id), Some(false), &user)
                .await
        })
        .await
    }

    pub async fn update_user(
        &self,
        user: types::User,
        update_mask: String,
    ) -> Result<types::User, ApiError> {
        let user_id = resource_id(
            user.name
                .as_deref()
                .ok_or(ApiError::MissingField("user name"))?,
        )
        .to_string();
        self.execute(move |client| async move {
            client
                .user_service_update_user(&user_id, Some(false), Some(&update_mask), &user)
                .await
        })
        .await
    }

    pub async fn delete_user(&self, user_name: String, force: bool) -> Result<(), ApiError> {
        let user_id = resource_id(&user_name).to_string();
        self.execute(move |client| async move {
            client.user_service_delete_user(&user_id, Some(force)).await
        })
        .await
    }

    pub async fn get_user_stats(&self, user_name: String) -> Result<types::UserStats, ApiError> {
        let user_id = resource_id(&user_name).to_string();
        self.execute(
            move |client| async move { client.user_service_get_user_stats(&user_id).await },
        )
        .await
    }

    pub async fn list_all_user_stats(&self) -> Result<types::ListAllUserStatsResponse, ApiError> {
        self.execute(move |client| async move {
            client
                .user_service_list_all_user_stats(
                    None,
                    Some(types::UserServiceListAllUserStatsState::Normal),
                )
                .await
        })
        .await
    }

    pub async fn list_user_settings(
        &self,
        user_name: String,
    ) -> Result<Vec<types::UserSetting>, ApiError> {
        let user_id = resource_id(&user_name).to_string();
        let mut settings = Vec::new();
        let mut page_token = None;
        loop {
            let response = self
                .execute({
                    let user_id = user_id.clone();
                    let page_token = page_token.clone();
                    move |client| async move {
                        client
                            .user_service_list_user_settings(
                                &user_id,
                                Some(100),
                                page_token.as_deref(),
                            )
                            .await
                    }
                })
                .await?;
            settings.extend(response.settings);
            page_token = response.next_page_token.filter(|token| !token.is_empty());
            if page_token.is_none() {
                return Ok(settings);
            }
        }
    }

    pub async fn update_user_setting(
        &self,
        setting: types::UserSetting,
        update_mask: Option<String>,
    ) -> Result<types::UserSetting, ApiError> {
        let name = setting
            .name
            .as_deref()
            .ok_or(ApiError::MissingField("user setting name"))?;
        let (user_id, setting_id) = nested_resource_ids(name, "users", "settings", "user setting")?;
        self.execute(move |client| async move {
            client
                .user_service_update_user_setting(
                    &user_id,
                    &setting_id,
                    update_mask.as_deref(),
                    &setting,
                )
                .await
        })
        .await
    }

    pub async fn list_linked_identities(
        &self,
        user_name: String,
    ) -> Result<Vec<types::LinkedIdentity>, ApiError> {
        let user_id = resource_id(&user_name).to_string();
        Ok(self
            .execute(move |client| async move {
                client.user_service_list_linked_identities(&user_id).await
            })
            .await?
            .linked_identities)
    }

    pub async fn create_linked_identity(
        &self,
        user_name: String,
        request: types::CreateLinkedIdentityRequest,
    ) -> Result<types::LinkedIdentity, ApiError> {
        let user_id = resource_id(&user_name).to_string();
        self.execute(move |client| async move {
            client
                .user_service_create_linked_identity(&user_id, &request)
                .await
        })
        .await
    }

    pub async fn delete_linked_identity(&self, identity_name: String) -> Result<(), ApiError> {
        let (user_id, identity_id) = nested_resource_ids(
            &identity_name,
            "users",
            "linkedIdentities",
            "linked identity",
        )?;
        self.execute(move |client| async move {
            client
                .user_service_delete_linked_identity(&user_id, &identity_id)
                .await
        })
        .await
    }

    pub async fn list_access_tokens(
        &self,
        user_name: String,
    ) -> Result<Vec<types::PersonalAccessToken>, ApiError> {
        let user_id = resource_id(&user_name).to_string();
        let mut tokens = Vec::new();
        let mut page_token = None;
        loop {
            let response = self
                .execute({
                    let user_id = user_id.clone();
                    let page_token = page_token.clone();
                    move |client| async move {
                        client
                            .user_service_list_personal_access_tokens(
                                &user_id,
                                Some(100),
                                page_token.as_deref(),
                            )
                            .await
                    }
                })
                .await?;
            tokens.extend(response.personal_access_tokens);
            page_token = response.next_page_token.filter(|token| !token.is_empty());
            if page_token.is_none() {
                return Ok(tokens);
            }
        }
    }

    pub async fn create_access_token(
        &self,
        user_name: String,
        request: types::CreatePersonalAccessTokenRequest,
    ) -> Result<types::CreatePersonalAccessTokenResponse, ApiError> {
        let user_id = resource_id(&user_name).to_string();
        self.execute(move |client| async move {
            client
                .user_service_create_personal_access_token(&user_id, &request)
                .await
        })
        .await
    }

    pub async fn delete_access_token(&self, token_name: String) -> Result<(), ApiError> {
        let (user_id, token_id) =
            nested_resource_ids(&token_name, "users", "personalAccessTokens", "access token")?;
        self.execute(move |client| async move {
            client
                .user_service_delete_personal_access_token(&user_id, &token_id)
                .await
        })
        .await
    }

    pub async fn list_webhooks(
        &self,
        user_name: String,
    ) -> Result<Vec<types::UserWebhook>, ApiError> {
        let user_id = resource_id(&user_name).to_string();
        Ok(self
            .execute(
                move |client| async move { client.user_service_list_user_webhooks(&user_id).await },
            )
            .await?
            .webhooks)
    }

    pub async fn create_webhook(
        &self,
        user_name: String,
        webhook: types::UserWebhook,
    ) -> Result<types::UserWebhook, ApiError> {
        let user_id = resource_id(&user_name).to_string();
        self.execute(move |client| async move {
            client
                .user_service_create_user_webhook(&user_id, &webhook)
                .await
        })
        .await
    }

    pub async fn update_webhook(
        &self,
        webhook: types::UserWebhook,
    ) -> Result<types::UserWebhook, ApiError> {
        let name = webhook
            .name
            .as_deref()
            .ok_or(ApiError::MissingField("webhook name"))?;
        let (user_id, webhook_id) = nested_resource_ids(name, "users", "webhooks", "webhook")?;
        let update_mask = if webhook.signing_secret.is_some() {
            "display_name,url,signing_secret"
        } else {
            "display_name,url"
        };
        self.execute(move |client| async move {
            client
                .user_service_update_user_webhook(
                    &user_id,
                    &webhook_id,
                    Some(update_mask),
                    &webhook,
                )
                .await
        })
        .await
    }

    pub async fn delete_webhook(&self, webhook_name: String) -> Result<(), ApiError> {
        let (user_id, webhook_id) =
            nested_resource_ids(&webhook_name, "users", "webhooks", "webhook")?;
        self.execute(move |client| async move {
            client
                .user_service_delete_user_webhook(&user_id, &webhook_id)
                .await
        })
        .await
    }

    pub async fn get_webhook_secret(&self, webhook_name: String) -> Result<String, ApiError> {
        let (user_id, webhook_id) =
            nested_resource_ids(&webhook_name, "users", "webhooks", "webhook")?;
        Ok(self
            .execute(move |client| async move {
                client
                    .user_service_get_user_webhook_signing_secret(&user_id, &webhook_id)
                    .await
            })
            .await?
            .signing_secret
            .unwrap_or_default())
    }

    pub async fn list_instance_settings(
        &self,
        names: Vec<String>,
    ) -> Result<Vec<types::InstanceSetting>, ApiError> {
        let request = types::BatchGetInstanceSettingsRequest { names };
        Ok(self
            .execute(move |client| async move {
                client
                    .instance_service_batch_get_instance_settings(&request)
                    .await
            })
            .await?
            .settings)
    }

    pub async fn update_instance_setting(
        &self,
        setting: types::InstanceSetting,
        update_mask: Option<String>,
    ) -> Result<types::InstanceSetting, ApiError> {
        let setting_id = resource_id(
            setting
                .name
                .as_deref()
                .ok_or(ApiError::MissingField("instance setting name"))?,
        )
        .to_string();
        self.execute(move |client| async move {
            client
                .instance_service_update_instance_setting(
                    &setting_id,
                    update_mask.as_deref(),
                    &setting,
                )
                .await
        })
        .await
    }

    pub async fn test_email_setting(
        &self,
        request: types::TestInstanceEmailSettingRequest,
    ) -> Result<(), ApiError> {
        self.execute(move |client| async move {
            client
                .instance_service_test_instance_email_setting(&request)
                .await
        })
        .await
    }

    pub async fn get_instance_stats(&self) -> Result<types::InstanceStats, ApiError> {
        self.execute(
            move |client| async move { client.instance_service_get_instance_stats().await },
        )
        .await
    }

    pub async fn list_identity_providers(&self) -> Result<Vec<types::IdentityProvider>, ApiError> {
        Ok(self
            .execute(move |client| async move {
                client
                    .identity_provider_service_list_identity_providers()
                    .await
            })
            .await?
            .identity_providers)
    }

    pub async fn create_identity_provider(
        &self,
        provider: types::IdentityProvider,
        provider_id: Option<String>,
    ) -> Result<types::IdentityProvider, ApiError> {
        self.execute(move |client| async move {
            client
                .identity_provider_service_create_identity_provider(
                    provider_id.as_deref(),
                    &provider,
                )
                .await
        })
        .await
    }

    pub async fn update_identity_provider(
        &self,
        provider: types::IdentityProvider,
    ) -> Result<types::IdentityProvider, ApiError> {
        let provider_id = resource_id(
            provider
                .name
                .as_deref()
                .ok_or(ApiError::MissingField("identity provider name"))?,
        )
        .to_string();
        self.execute(move |client| async move {
            client
                .identity_provider_service_update_identity_provider(
                    &provider_id,
                    Some("title,identifier_filter,config"),
                    &provider,
                )
                .await
        })
        .await
    }

    pub async fn delete_identity_provider(&self, provider_name: String) -> Result<(), ApiError> {
        let provider_id = resource_id(&provider_name).to_string();
        self.execute(move |client| async move {
            client
                .identity_provider_service_delete_identity_provider(&provider_id)
                .await
        })
        .await
    }
}

fn nested_resource_ids(
    name: &str,
    root: &str,
    collection: &str,
    label: &str,
) -> Result<(String, String), ApiError> {
    match name.split('/').collect::<Vec<_>>().as_slice() {
        [actual_root, parent_id, actual_collection, child_id]
            if *actual_root == root
                && *actual_collection == collection
                && !parent_id.is_empty()
                && !child_id.is_empty() =>
        {
            Ok(((*parent_id).to_string(), (*child_id).to_string()))
        }
        _ => Err(ApiError::Request(format!(
            "invalid {label} resource name: {name}"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_resource_parser_requires_the_exact_shape() {
        assert_eq!(
            nested_resource_ids("users/alice/webhooks/hook", "users", "webhooks", "webhook")
                .unwrap(),
            ("alice".into(), "hook".into())
        );
        assert!(
            nested_resource_ids(
                "users/alice/webhooks/hook/extra",
                "users",
                "webhooks",
                "webhook"
            )
            .is_err()
        );
        assert!(
            nested_resource_ids("memos/alice/webhooks/hook", "users", "webhooks", "webhook")
                .is_err()
        );
    }
}
