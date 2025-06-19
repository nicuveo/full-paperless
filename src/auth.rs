use base64::engine::{Engine, general_purpose::STANDARD};
use secure_string::SecureString;

#[derive(Debug, Clone)]
pub enum Auth {
    Basic {
        username: SecureString,
        password: SecureString,
    },
    Token(SecureString),
}

impl Auth {
    #[must_use]
    pub fn header_value(&self) -> String {
        match self {
            Auth::Basic { username, password } => format!(
                "Basic {}",
                STANDARD.encode(format!("{}:{}", username.unsecure(), password.unsecure())),
            ),
            Auth::Token(token) => format!("Token {}", token.unsecure()),
        }
    }
}
