#[derive(Debug, Clone)]
pub struct MultiFactorAuth {
    pub user: String,
    pub factor_token: String,
    pub token_expiration: u64,
}

impl MultiFactorAuth {
    pub fn new(user: &str, factor_token: &str, token_expiration: u64) -> Self {
        MultiFactorAuth {
            user: user.to_string(),
            factor_token: factor_token.to_string(),
            token_expiration,
        }
    }

    pub fn verify_token(&self, token: &str) -> bool {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() < self.token_expiration && self.factor_token == token
    }
}
