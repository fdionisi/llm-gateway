use axum::async_trait;
use jsonwebtoken::{
    decode, decode_header,
    jwk::{AlgorithmParameters, JwkSet},
    DecodingKey, TokenData, Validation,
};

use super::auth_error::AuthError;

#[async_trait]
pub trait AuthProvider {
    /// Returns the JWK set for the provider.
    async fn jwk_set(&self) -> Result<JwkSet, AuthError>;

    fn decode_validation(&self, validation: Validation) -> Validation {
        validation
    }

    /// Verifies the token and returns the claims.
    async fn verify(&self, token: &str) -> Result<TokenData<Claims>, AuthError> {
        let token_sections: Vec<&str> = token.split('.').collect();
        if token_sections.len() < 2 {
            return Err(AuthError::InvalidToken);
        }

        let header = decode_header(&token).map_err(|_| AuthError::InvalidToken)?;

        let jwk_set = self.jwk_set().await?;

        let Some(kid) = header.kid else {
            // return Err("Token doesn't have a `kid` header field".into());
            return Err(AuthError::InvalidToken);
        };

        let Some(jwk) = jwk_set.find(&kid) else {
            // return Err("No matching JWK found for the given kid".into());
            return Err(AuthError::InvalidToken);
        };

        // todo!(flexible algorithm selection)
        let decoding_key = match &jwk.algorithm {
            AlgorithmParameters::RSA(rsa) => DecodingKey::from_rsa_components(&rsa.n, &rsa.e)
                .map_err(|_| AuthError::InvalidToken)?,
            _ => unreachable!("algorithm should be a RSA in this example"),
        };

        let validation = self.decode_validation(Validation::new(header.alg));

        Ok(decode::<Claims>(token, &decoding_key, &validation)
            .map_err(|_| AuthError::InvalidToken)?)
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}
