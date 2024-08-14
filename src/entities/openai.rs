use async_openai::types::{CreateChatCompletionRequest, CreateChatCompletionResponse};

use super::{CreateCompletionRequest, CreateCompletionResponse};

impl From<CreateChatCompletionRequest> for CreateCompletionRequest {
    fn from(chat_request: CreateChatCompletionRequest) -> Self {
        serde_json::from_value(serde_json::to_value(chat_request).unwrap()).unwrap()
    }
}

impl Into<CreateChatCompletionRequest> for CreateCompletionRequest {
    fn into(self) -> CreateChatCompletionRequest {
        serde_json::from_value(serde_json::to_value(self).unwrap()).unwrap()
    }
}

impl From<CreateChatCompletionResponse> for CreateCompletionResponse {
    fn from(chat_request: CreateChatCompletionResponse) -> Self {
        serde_json::from_value(serde_json::to_value(chat_request).unwrap()).unwrap()
    }
}

impl Into<CreateChatCompletionResponse> for CreateCompletionResponse {
    fn into(self) -> CreateChatCompletionResponse {
        serde_json::from_value(serde_json::to_value(self).unwrap()).unwrap()
    }
}
