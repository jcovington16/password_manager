# Password Manager

A simple command-line password manager written in Rust. This tool allows users to securely store and retrieve passwords associated with URLs using encryption.

## Features

- **Secure Storage:** Saves passwords in an encrypted format.
- **Password Retrieval:** Fetches stored credentials by URL.
- **File-Based Storage:** Stores credentials in a local file (`password_manager.txt`).
- **User-Friendly Menu:** Provides an interactive command-line interface.

## Dependencies

This project uses the following Rust crates:
- [`dialoguer`](https://crates.io/crates/dialoguer) - For interactive CLI prompts.
- [`magic-crypt`](https://crates.io/crates/magic-crypt) - For password encryption.

## Installation

1. **Clone the repository:**
   ```sh
   git clone https://github.com/your-repo/password-manager.git
   cd password-manager

2. **Install Rust if you haven't already:**
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

3. **Build the project:**
    cargo build --release

4. **Run the password manager:**
    cargo run

## Usage
- Upon running the application, you will be prompted with three options:

    1. New Password - Store a new password by providing a URL, username, and password.
    2. Grab Password - Retrieve a stored password by providing the URL.
    3. Exit - Close the application.

### Adding a Password
- Select New Password.
- Enter the URL, username, and password.
- The password will be encrypted and saved in password_manager.txt.

### Retrieving a Password
- Select Grab Password.
- Enter the URL for which you want to retrieve credentials.
- The stored credentials will be displayed.

### File Storage
- All credentials are saved in password_manager.txt. Passwords are stored in an encrypted format, ensuring security.

### Security Considerations
- The encryption key (magickey) is hardcoded. Consider making it configurable via an environment variable.
- Ensure the password_manager.txt file is kept secure and inaccessible to unauthorized users.
- Future improvements could include hashing and securely managing encryption keys.

### Future Improvements
- Environment Variable for Encryption Key: Instead of hardcoding the encryption key, use an environment variable for better security.
- Decryption Functionality: Implement a function to decrypt and display stored passwords.
- Improved File Handling: Use a structured file format like JSON or SQLite for better data management.
- Secure Password Input: Utilize rpassword crate to hide password input when entering a new password.
- User Authentication: Implement a master password to access stored credentials.
- Cross-Platform Support: Ensure compatibility with Windows, macOS, and Linux.

### License
This project is open-source and available under the MIT License.