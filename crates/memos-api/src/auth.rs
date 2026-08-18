use std::{
    collections::HashMap,
    sync::{OnceLock, RwLock},
};

use reqwest::header::HeaderValue;

use crate::{Client, Error};
use progenitor_client::{ClientHooks, OperationInfo};

static TOKENS: OnceLock<RwLock<HashMap<String, HeaderValue>>> = OnceLock::new();

fn tokens() -> &'static RwLock<HashMap<String, HeaderValue>> {
    TOKENS.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Sets the bearer token used by all clones of a client for the same base URL.
pub fn set_access_token(
    client: &Client,
    token: &str,
) -> Result<(), reqwest::header::InvalidHeaderValue> {
    let mut value = HeaderValue::from_str(&format!("Bearer {token}"))?;
    value.set_sensitive(true);
    tokens()
        .write()
        .expect("access token registry poisoned")
        .insert(client.baseurl.clone(), value);
    Ok(())
}

/// Clears the bearer token associated with the client's base URL.
pub fn clear_access_token(client: &Client) {
    tokens()
        .write()
        .expect("access token registry poisoned")
        .remove(&client.baseurl);
}

fn access_token(client: &Client) -> Option<HeaderValue> {
    tokens()
        .read()
        .expect("access token registry poisoned")
        .get(&client.baseurl)
        .cloned()
}

impl ClientHooks<()> for Client {
    async fn pre<E>(
        &self,
        request: &mut reqwest::Request,
        _info: &OperationInfo,
    ) -> Result<(), Error<E>> {
        if let Some(token) = access_token(self) {
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

    #[test]
    fn bearer_token_is_injected_into_generated_requests() {
        let client = Client::new("https://memos.example.com");
        set_access_token(&client, "token-123").unwrap();
        let mut request = reqwest::Request::new(
            reqwest::Method::GET,
            "https://memos.example.com/api/v1/auth/me".parse().unwrap(),
        );
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
        assert!(
            request
                .headers()
                .get(reqwest::header::AUTHORIZATION)
                .unwrap()
                .is_sensitive()
        );
        clear_access_token(&client);
    }
}
