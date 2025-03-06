use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Credential {
    pub username: String,
    pub password: String,
}

// Encrypts the password
pub fn encrypt_str(password: &str) -> String {
    let mc = new_magic_crypt!("magickey", 256);
    mc.encrypt_str_to_base64(password)
}

// Adds user credentials to the HashMap (Testable Function)
pub fn add_user_info_with_input(
    credentials: &mut HashMap<String, Credential>,
    url: &str,
    username: &str,
    password: &str,
) {
    let encrypted_password = encrypt_str(&password.to_string());

    credentials.insert(
        url.to_string(),
        Credential {
            username: username.to_string(),
            password: encrypted_password,
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_user_info() {
        let mut credentials = HashMap::new();
        let url = "example.com";
        let username = "test_user";
        let password = "secure_password";

        add_user_info_with_input(&mut credentials, url, username, password);

        assert!(credentials.contains_key(url));

        let stored_credential = credentials.get(url).unwrap();
        assert_eq!(stored_credential.username, username);
        assert_ne!(stored_credential.password, password); // Ensure password is encrypted
    }
}
