use std::sync::Arc;

use axum::async_trait;

use super::{
    secret_manager_error::SecretManagerError, secret_manager_provider::SecretManagerProvider,
};

pub struct AwsSecretManagerProvider {
    region: String,
}

pub struct AwsSecretManagerProviderBuilder {
    region: Option<String>,
}

impl AwsSecretManagerProvider {
    pub fn builder() -> AwsSecretManagerProviderBuilder {
        AwsSecretManagerProviderBuilder { region: None }
    }
}

impl AwsSecretManagerProviderBuilder {
    pub fn region(mut self, region: String) -> Self {
        self.region = Some(region);
        self
    }

    pub fn build(self) -> Arc<AwsSecretManagerProvider> {
        Arc::new(AwsSecretManagerProvider {
            region: self.region.unwrap(),
        })
    }
}

#[async_trait]
impl SecretManagerProvider for AwsSecretManagerProvider {
    async fn secret(&self, secret_id: &str) -> Result<String, SecretManagerError> {
        Ok("secret".to_string())
    }
}
