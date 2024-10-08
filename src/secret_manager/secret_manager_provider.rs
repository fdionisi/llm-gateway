use axum::async_trait;

use super::secret_manager_error::SecretManagerError;

#[async_trait]
pub trait SecretManagerProvider: Send + Sync {
    async fn secret(&self, secret_id: &str) -> Result<String, SecretManagerError>;
}
