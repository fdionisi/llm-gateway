#[derive(Clone, Debug, thiserror::Error)]
pub enum SecretManagerError {
    #[error("Secret not found")]
    NotFound,
}
