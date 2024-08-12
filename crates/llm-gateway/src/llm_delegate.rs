use std::sync::Arc;

use async_openai::{
    config::OpenAIConfig,
    error::OpenAIError,
    types::{
        CreateChatCompletionRequest, CreateChatCompletionResponse, CreateEmbeddingRequest,
        CreateEmbeddingResponse,
    },
};
use tokio::sync::Mutex;

use super::secret_manager::SecretManagerProvider;

pub enum SupportedLlm {
    OpenAi,
}

#[derive(Clone)]
pub struct LlmDelegate {
    secret_manager: Arc<dyn SecretManagerProvider + Send + Sync>,
    openai: Arc<Mutex<Option<async_openai::Client<OpenAIConfig>>>>,
}

impl LlmDelegate {
    pub fn new(secret_manager: Arc<dyn SecretManagerProvider + Send + Sync>) -> Self {
        Self {
            secret_manager,
            openai: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn completion(
        &self,
        llm: SupportedLlm,
        request: CreateChatCompletionRequest,
    ) -> Result<CreateChatCompletionResponse, OpenAIError> {
        match llm {
            SupportedLlm::OpenAi => Ok(self.ensure_openai().await?.chat().create(request).await?),
        }
    }

    pub async fn embeddings(
        &self,
        llm: SupportedLlm,
        request: CreateEmbeddingRequest,
    ) -> Result<CreateEmbeddingResponse, OpenAIError> {
        match llm {
            SupportedLlm::OpenAi => Ok(self
                .ensure_openai()
                .await?
                .embeddings()
                .create(request)
                .await?),
        }
    }

    async fn ensure_openai(&self) -> Result<async_openai::Client<OpenAIConfig>, OpenAIError> {
        let secret = self.secret_manager.secret("OPENAI_API_KEY").await.unwrap();
        let mut openai = self.openai.lock().await;
        if openai.is_none() {
            *openai = Some(async_openai::Client::with_config(
                OpenAIConfig::new().with_api_key(secret),
            ));
        }
        Ok(openai.as_ref().unwrap().clone())
    }
}
