use std::sync::Arc;

use axum::async_trait;
use jsonwebtoken::{jwk::JwkSet, Validation};
use tokio::sync::Mutex;

use super::{auth_error::AuthError, auth_provider::AuthProvider};

pub struct Auth0AuthProvider {
    issuer: String,
    audience: String,
    cached_keys: Arc<Mutex<Option<JwkSet>>>,
}

pub struct Auth0AuthProviderBuilder {
    issuer: Option<String>,
    audience: Option<String>,
}

impl Auth0AuthProvider {
    pub fn builder() -> Auth0AuthProviderBuilder {
        Auth0AuthProviderBuilder {
            issuer: None,
            audience: None,
        }
    }
}

impl Auth0AuthProviderBuilder {
    pub fn issuer(mut self, issuer: String) -> Self {
        self.issuer = Some(issuer);
        self
    }

    pub fn audience(mut self, audience: String) -> Self {
        self.audience = Some(audience);
        self
    }

    pub fn build(self) -> Arc<Auth0AuthProvider> {
        Arc::new(Auth0AuthProvider {
            issuer: self.issuer.unwrap_or_else(|| {
                std::env::var("AUTH0_ISSUER_BASE_URL").expect("issuer is required")
            }),
            audience: self
                .audience
                .unwrap_or_else(|| std::env::var("AUTH0_AUDIENCE").expect("audience is required")),
            cached_keys: Arc::new(Mutex::new(None)),
        })
    }
}

#[async_trait]
impl AuthProvider for Auth0AuthProvider {
    async fn jwk_set(&self) -> Result<JwkSet, AuthError> {
        let mut cached_keys = self.cached_keys.lock().await;
        if cached_keys.is_none() {
            let client = reqwest::Client::new();
            let jwk_set = client
                .get(format!("{}/.well-known/jwks.json", self.issuer))
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
        validation.set_audience(&[&self.audience]);
        validation.validate_exp = true;
        validation
    }
}
