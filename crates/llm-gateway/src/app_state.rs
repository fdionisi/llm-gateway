use std::sync::Arc;

use axum::extract::FromRef;

use crate::{auth::AuthProvider, llm_delegate::LlmDelegate};

#[derive(Clone)]
pub struct AppState {
    llm_delegate: LlmDelegate,
    auth_provider: Arc<dyn AuthProvider + Send + Sync>,
}

impl AppState {
    pub fn new(
        llm_delegate: LlmDelegate,
        auth_provider: Arc<dyn AuthProvider + Send + Sync>,
    ) -> Self {
        Self {
            llm_delegate,
            auth_provider,
        }
    }
}

impl FromRef<AppState> for Arc<dyn AuthProvider + Send + Sync> {
    fn from_ref(app_state: &AppState) -> Arc<dyn AuthProvider + Send + Sync> {
        app_state.auth_provider.clone()
    }
}

impl FromRef<AppState> for LlmDelegate {
    fn from_ref(app_state: &AppState) -> LlmDelegate {
        app_state.llm_delegate.clone()
    }
}
