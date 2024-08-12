use std::sync::Arc;

use axum::async_trait;
use jsonwebtoken::{jwk::JwkSet, Validation};
use tokio::sync::Mutex;

use super::{auth_error::AuthError, auth_provider::AuthProvider};

pub struct CognitoAuthProvider {
    user_pool_id: String,
    region: String,
    cached_keys: Arc<Mutex<Option<JwkSet>>>,
}

pub struct CognitoAuthProviderBuilder {
    user_pool_id: Option<String>,
    region: Option<String>,
}

impl CognitoAuthProvider {
    pub fn builder() -> CognitoAuthProviderBuilder {
        CognitoAuthProviderBuilder {
            user_pool_id: None,
            region: None,
        }
    }
}

impl CognitoAuthProviderBuilder {
    pub fn user_pool_id(mut self, user_pool_id: String) -> Self {
        self.user_pool_id = Some(user_pool_id);
        self
    }

    pub fn region(mut self, region: String) -> Self {
        self.region = Some(region);
        self
    }

    pub fn build(self) -> Arc<CognitoAuthProvider> {
        Arc::new(CognitoAuthProvider {
            user_pool_id: self.user_pool_id.expect("user_pool_id is required"),
            region: self.region.expect("region is required"),
            cached_keys: Arc::new(Mutex::new(None)),
        })
    }
}

impl CognitoAuthProvider {
    fn issuer(&self) -> String {
        format!(
            "https://cognito-idp.{}.amazonaws.com/{}",
            self.region, self.user_pool_id
        )
    }
}

#[async_trait]
impl AuthProvider for CognitoAuthProvider {
    async fn jwk_set(&self) -> Result<JwkSet, AuthError> {
        let mut cached_keys = self.cached_keys.lock().await;
        if cached_keys.is_none() {
            let client = reqwest::Client::new();
            let jwk_set = client
                .get(format!("{}/.well-known/jwks.json", self.issuer()))
                .send()
                .await
                .map_err(|_| AuthError::MissingCredentials)?
                .json::<JwkSet>()
                .await
                .map_err(|_| AuthError::MissingCredentials)?;

            *cached_keys = Some(jwk_set);
        }

        Ok(cached_keys.as_ref().unwrap().to_owned())
    }

    fn decode_validation(&self, mut validation: Validation) -> Validation {
        validation.validate_exp = true;
        validation
    }
}
