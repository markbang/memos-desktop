use keyring::{Entry, Error};
use sha2::{Digest as _, Sha256};

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

fn entry(server_url: &str, username: &str) -> Result<Entry, String> {
    Entry::new(SERVICE, &account_key(server_url, username)).map_err(|error| error.to_string())
}

fn account_key(server_url: &str, username: &str) -> String {
    let digest = Sha256::digest(server_url.trim_end_matches('/').as_bytes());
    let server = digest[..12]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{server}:{username}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn credential_account_is_stable_and_server_scoped() {
        assert_eq!(
            account_key("https://memos.example.com/", "alice"),
            account_key("https://memos.example.com", "alice")
        );
        assert_ne!(
            account_key("https://one.example.com", "alice"),
            account_key("https://two.example.com", "alice")
        );
    }
}
