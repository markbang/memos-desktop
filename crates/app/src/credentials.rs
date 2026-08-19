use keyring::{Entry, Error};
use sha2::{Digest as _, Sha256};
use url::Url;

const SERVICE: &str = "com.markbang.MemosDesktop.password";

pub fn load_password(server_url: &str, username: &str) -> Result<Option<String>, String> {
    let entry = entry(server_url, username)?;
    match entry.get_password() {
        Ok(password) => Ok(Some(password)),
        Err(Error::NoEntry) => Ok(None),
        Err(error) => Err(error.to_string()),
    }
}

pub fn save_password(server_url: &str, username: &str, password: &str) -> Result<(), String> {
    entry(server_url, username)?
        .set_password(password)
        .map_err(|error| error.to_string())
}

pub fn delete_password(server_url: &str, username: &str) -> Result<(), String> {
    let entry = entry(server_url, username)?;
    match entry.delete_credential() {
        Ok(()) | Err(Error::NoEntry) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

pub fn migrate_password(
    server_url: &str,
    old_username: &str,
    new_username: &str,
) -> Result<bool, String> {
    if old_username == new_username {
        return Ok(false);
    }
    let Some(password) = load_password(server_url, old_username)? else {
        return Ok(false);
    };
    save_password(server_url, new_username, &password)?;
    delete_password(server_url, old_username)?;
    Ok(true)
}

fn entry(server_url: &str, username: &str) -> Result<Entry, String> {
    Entry::new(SERVICE, &account_key(server_url, username)?).map_err(|error| error.to_string())
}

fn account_key(server_url: &str, username: &str) -> Result<String, String> {
    let mut url = Url::parse(server_url.trim()).map_err(|error| error.to_string())?;
    url.set_query(None);
    url.set_fragment(None);
    let canonical_url = url.as_str().trim_end_matches('/');
    let digest = Sha256::digest(canonical_url.as_bytes());
    let server = digest[..12]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    Ok(format!("{server}:{username}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn credential_account_is_stable_and_server_scoped() {
        assert_eq!(
            account_key("HTTPS://MEMOS.EXAMPLE.COM/", "alice").unwrap(),
            account_key("https://memos.example.com", "alice").unwrap()
        );
        assert_ne!(
            account_key("https://one.example.com", "alice").unwrap(),
            account_key("https://two.example.com", "alice").unwrap()
        );
    }
}
