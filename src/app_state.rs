use axum::extract::FromRef;

use crate::llm_delegate::LlmDelegate;

#[derive(Clone)]
pub struct AppState {
    llm_delegate: LlmDelegate,
    token: String,
}

impl AppState {
    pub fn new(llm_delegate: LlmDelegate, token: String) -> Self {
        Self {
            llm_delegate,
            token: token.to_string(),
        }
    }
}

impl FromRef<AppState> for String {
    fn from_ref(app_state: &AppState) -> String {
        app_state.token.clone()
    }
}

impl FromRef<AppState> for LlmDelegate {
    fn from_ref(app_state: &AppState) -> LlmDelegate {
        app_state.llm_delegate.clone()
    }
}
