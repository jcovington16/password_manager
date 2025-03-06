use password_manager::add_user_info_with_input; // Import the function
use password_manager::Credential;
use std::collections::HashMap;

#[test]
fn test_add_user_info_integration() {
    let mut credentials = HashMap::new();
    let url = "example.com";
    let username = "test_user";
    let password = "secure_password";

    add_user_info_with_input(&mut credentials, url, username, password);

    assert!(credentials.contains_key(url));

    let stored_credential = credentials.get(url).unwrap();
    assert_eq!(stored_credential.username, username);
    assert_ne!(stored_credential.password, password); // Password should be encrypted
}
