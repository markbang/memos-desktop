use std::{
    io::{Read, Write},
    net::TcpListener,
    time::{Duration, Instant},
};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use memos_api::types::IdentityProvider;
use rand::{Rng as _, distr::Alphanumeric};
use sha2::{Digest as _, Sha256};
use url::Url;

use crate::api::ApiError;

pub struct SsoFlow {
    pub authorize_url: String,
    pub code_verifier: String,
    pub idp_name: String,
    pub redirect_uri: String,
    state: String,
    listener: TcpListener,
}

impl SsoFlow {
    pub fn prepare(provider: &IdentityProvider) -> Result<Self, ApiError> {
        let idp_name = provider
            .name
            .clone()
            .ok_or(ApiError::MissingField("identity provider name"))?;
        let config = provider
            .config
            .oauth2_config
            .as_ref()
            .ok_or(ApiError::MissingField("OAuth2 provider config"))?;
        let auth_url = config
            .auth_url
            .as_ref()
            .ok_or(ApiError::MissingField("OAuth2 authorization URL"))?;
        let client_id = config
            .client_id
            .as_ref()
            .ok_or(ApiError::MissingField("OAuth2 client ID"))?;
        let listener = TcpListener::bind("127.0.0.1:0")
            .map_err(|error| ApiError::Request(format!("failed to bind SSO callback: {error}")))?;
        listener
            .set_nonblocking(true)
            .map_err(|error| ApiError::Request(error.to_string()))?;
        let redirect_uri = format!(
            "http://127.0.0.1:{}/callback",
            listener
                .local_addr()
                .map_err(|error| ApiError::Request(error.to_string()))?
                .port()
        );
        let code_verifier = random_token(64);
        let state = random_token(32);
        let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(code_verifier.as_bytes()));
        let mut authorize_url = Url::parse(auth_url).map_err(|error| {
            ApiError::Request(format!("invalid OAuth2 authorization URL: {error}"))
        })?;
        authorize_url
            .query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", client_id)
            .append_pair("redirect_uri", &redirect_uri)
            .append_pair("scope", &config.scopes.join(" "))
            .append_pair("state", &state)
            .append_pair("code_challenge", &challenge)
            .append_pair("code_challenge_method", "S256");

        Ok(Self {
            authorize_url: authorize_url.to_string(),
            code_verifier,
            idp_name,
            redirect_uri,
            state,
            listener,
        })
    }

    pub fn wait_for_callback(self) -> Result<SsoCallback, ApiError> {
        let deadline = Instant::now() + Duration::from_secs(300);
        'accept: loop {
            match self.listener.accept() {
                Ok((mut stream, _)) => {
                    stream
                        .set_nonblocking(false)
                        .map_err(|error| ApiError::Request(error.to_string()))?;
                    stream
                        .set_read_timeout(Some(Duration::from_secs(5)))
                        .map_err(|error| ApiError::Request(error.to_string()))?;
                    let mut request = [0_u8; 8192];
                    let mut read = 0;
                    loop {
                        match stream.read(&mut request[read..]) {
                            Ok(0) => break,
                            Ok(count) => {
                                read += count;
                                if Instant::now() >= deadline {
                                    return Err(ApiError::Request("SSO callback timed out".into()));
                                }
                                if request[..read]
                                    .windows(4)
                                    .any(|window| window == b"\r\n\r\n")
                                    || request[..read].windows(2).any(|window| window == b"\n\n")
                                    || read == request.len()
                                {
                                    break;
                                }
                            }
                            Err(error)
                                if matches!(
                                    error.kind(),
                                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                                ) =>
                            {
                                if Instant::now() >= deadline {
                                    return Err(ApiError::Request("SSO callback timed out".into()));
                                }
                                continue 'accept;
                            }
                            Err(error) => return Err(ApiError::Request(error.to_string())),
                        }
                    }
                    if read == 0 {
                        continue;
                    }
                    let request = String::from_utf8_lossy(&request[..read]);
                    let result =
                        parse_callback_code(&request, &self.state).map(|code| SsoCallback {
                            code,
                            code_verifier: self.code_verifier,
                            idp_name: self.idp_name,
                            redirect_uri: self.redirect_uri,
                        });
                    let (status, message) = match &result {
                        Ok(_) => (
                            "200 OK",
                            "Authentication complete. You can return to Memos Desktop.",
                        ),
                        Err(_) => (
                            "400 Bad Request",
                            "Authentication failed. Return to Memos Desktop for details.",
                        ),
                    };
                    let body =
                        format!("<!doctype html><html><body><h1>{message}</h1></body></html>");
                    let response = format!(
                        "HTTP/1.1 {status}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                        body.len()
                    );
                    let _ = stream.write_all(response.as_bytes());
                    return result;
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    if Instant::now() >= deadline {
                        return Err(ApiError::Request("SSO callback timed out".into()));
                    }
                    std::thread::sleep(Duration::from_millis(100));
                }
                Err(error) => return Err(ApiError::Request(error.to_string())),
            }
        }
    }
}

pub struct SsoCallback {
    pub code: String,
    pub code_verifier: String,
    pub idp_name: String,
    pub redirect_uri: String,
}

fn random_token(length: usize) -> String {
    rand::rng()
        .sample_iter(Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn parse_callback_code(request: &str, expected_state: &str) -> Result<String, ApiError> {
    let target = request
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .ok_or_else(|| ApiError::Request("invalid OAuth2 callback request".into()))?;
    let callback_url = Url::parse(&format!("http://127.0.0.1{target}"))
        .map_err(|error| ApiError::Request(error.to_string()))?;
    let mut code = None;
    let mut state = None;
    let mut oauth_error = None;
    for (key, value) in callback_url.query_pairs() {
        match key.as_ref() {
            "code" => code = Some(value.into_owned()),
            "state" => state = Some(value.into_owned()),
            "error" => oauth_error = Some(value.into_owned()),
            _ => {}
        }
    }
    if let Some(error) = oauth_error {
        return Err(ApiError::Request(format!(
            "OAuth2 provider returned: {error}"
        )));
    }
    if state.as_deref() != Some(expected_state) {
        return Err(ApiError::Request("OAuth2 state validation failed".into()));
    }
    code.ok_or(ApiError::MissingField("OAuth2 authorization code"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn callback_parser_requires_matching_state_and_code() {
        assert_eq!(
            parse_callback_code("GET /callback?state=state&code=code HTTP/1.1\r\n", "state")
                .unwrap(),
            "code"
        );
        assert!(
            parse_callback_code("GET /callback?state=wrong&code=code HTTP/1.1\r\n", "state")
                .is_err()
        );
        assert!(parse_callback_code("GET /callback?state=state HTTP/1.1\r\n", "state").is_err());
    }
}
