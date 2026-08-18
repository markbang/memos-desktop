use std::{
    fmt,
    sync::{Arc, RwLock},
};

use reqwest::header::HeaderValue;

use crate::{Client, Error};
use progenitor_client::{ClientHooks, OperationInfo};

#[derive(Default)]
pub(crate) struct AuthState {
    bearer: Option<HeaderValue>,
    refresh_cookie: Option<HeaderValue>,
}

impl fmt::Debug for AuthState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthState")
            .field("bearer", &self.bearer.is_some())
            .field("refresh_cookie", &self.refresh_cookie.is_some())
            .finish()
    }
}

pub(crate) type AuthorizationState = Arc<RwLock<AuthState>>;

/// Sets the bearer token used by this client and all of its clones.
pub fn set_access_token(
    client: &Client,
    token: &str,
) -> Result<(), reqwest::header::InvalidHeaderValue> {
    let mut value = HeaderValue::from_str(&format!("Bearer {token}"))?;
    value.set_sensitive(true);
    client
        .authorization
        .write()
        .expect("authorization lock poisoned")
        .bearer = Some(value);
    Ok(())
}

/// Sets the refresh cookie returned through Memos gRPC gateway metadata.
pub fn set_refresh_cookie(
    client: &Client,
    cookie: &str,
) -> Result<(), reqwest::header::InvalidHeaderValue> {
    let mut value = HeaderValue::from_str(cookie)?;
    value.set_sensitive(true);
    client
        .authorization
        .write()
        .expect("authorization lock poisoned")
        .refresh_cookie = Some(value);
    Ok(())
}

/// Clears all authentication state associated with this client and its clones.
pub fn clear_access_token(client: &Client) {
    *client
        .authorization
        .write()
        .expect("authorization lock poisoned") = AuthState::default();
}

/// Returns the current authorization header for long-lived transports such as SSE.
pub fn authorization_header(client: &Client) -> Option<HeaderValue> {
    client
        .authorization
        .read()
        .expect("authorization lock poisoned")
        .bearer
        .clone()
}

#[doc(hidden)]
pub fn refresh_cookie_header(client: &Client) -> Option<HeaderValue> {
    client
        .authorization
        .read()
        .expect("authorization lock poisoned")
        .refresh_cookie
        .clone()
}

impl Client {
    /// Returns the normalized API base URL.
    pub fn base_url(&self) -> &str {
        &self.baseurl
    }

    /// Returns the underlying HTTP client for non-OpenAPI transports.
    pub fn http_client(&self) -> &reqwest::Client {
        &self.client
    }
}

impl ClientHooks<()> for Client {
    async fn pre<E>(
        &self,
        request: &mut reqwest::Request,
        _info: &OperationInfo,
    ) -> Result<(), Error<E>> {
        let state = self
            .authorization
            .read()
            .expect("authorization lock poisoned");
        if let Some(token) = state.bearer.clone() {
            request
                .headers_mut()
                .insert(reqwest::header::AUTHORIZATION, token);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use progenitor_client::ClientHooks;

    fn request() -> reqwest::Request {
        reqwest::Request::new(
            reqwest::Method::GET,
            "https://memos.example.com/api/v1/auth/me".parse().unwrap(),
        )
    }

    #[test]
    fn bearer_header_is_injected_without_exposing_the_refresh_cookie() {
        let client = Client::new("https://memos.example.com");
        set_access_token(&client, "token-123").unwrap();
        set_refresh_cookie(&client, "memos_refresh=refresh-123").unwrap();
        let mut request = request();
        let info = OperationInfo {
            operation_id: "test",
        };

        tokio::runtime::Runtime::new().unwrap().block_on(async {
            <Client as ClientHooks<()>>::pre::<()>(&client, &mut request, &info)
                .await
                .unwrap();
        });

        assert_eq!(
            request
                .headers()
                .get(reqwest::header::AUTHORIZATION)
                .unwrap(),
            "Bearer token-123"
        );
        assert!(request.headers().get(reqwest::header::COOKIE).is_none());
        assert!(
            request
                .headers()
                .get(reqwest::header::AUTHORIZATION)
                .unwrap()
                .is_sensitive()
        );
        clear_access_token(&client);
    }

    #[test]
    fn sessions_for_the_same_server_keep_separate_tokens() {
        let first = Client::new("https://memos.example.com");
        let first_clone = first.clone();
        let second = Client::new("https://memos.example.com");

        set_access_token(&first, "first-token").unwrap();
        set_access_token(&second, "second-token").unwrap();

        assert_eq!(authorization_header(&first).unwrap(), "Bearer first-token");
        assert_eq!(
            authorization_header(&first_clone).unwrap(),
            "Bearer first-token"
        );
        assert_eq!(
            authorization_header(&second).unwrap(),
            "Bearer second-token"
        );

        clear_access_token(&first_clone);
        assert!(authorization_header(&first).is_none());
        assert_eq!(
            authorization_header(&second).unwrap(),
            "Bearer second-token"
        );
    }
}
