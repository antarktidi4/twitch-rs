#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub capabilities: String,
    pub broadcaster: String,
    pub username: String,
    pub token: String,
}

impl AuthConfig {
    pub fn from_env() -> Self {
        use std::env::var;

        Self {
            capabilities: var("CAPABILITIES").unwrap(),
            broadcaster: var("BROADCASTER").unwrap(),
            username: var("BOTNAME").unwrap(),
            token: var("TOKEN").unwrap(),
        }
    }
}
