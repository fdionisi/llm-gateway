use std::sync::Arc;

use axum::async_trait;

use super::{secret_manager_error::SecretManagerError, SecretManagerProvider};

pub struct EnvSecretManagerProvider;

impl EnvSecretManagerProvider {
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

#[async_trait]
impl SecretManagerProvider for EnvSecretManagerProvider {
    async fn secret(&self, secret_id: &str) -> Result<String, SecretManagerError> {
        std::env::var(secret_id).map_err(|_| SecretManagerError::NotFound)
    }
}
