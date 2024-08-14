use std::sync::Arc;

use axum::async_trait;
use futures::StreamExt;

use crate::{
    entities::{
        CompletionResponseStream, CreateCompletionRequest, CreateCompletionResponse, Model,
    },
    llm_delegate::SupportedLlm,
    secret_manager::SecretManagerProvider,
};

use super::{AnyLlmProvider, LlmProvider};

pub struct PerplexityAi(async_openai::Client<async_openai::config::OpenAIConfig>);

#[async_trait]
impl LlmProvider for PerplexityAi {
    async fn init(
        secret_manager: Arc<dyn SecretManagerProvider>,
    ) -> anyhow::Result<Arc<dyn AnyLlmProvider>> {
        let secret = secret_manager.secret("PERPLEXITYAI_API_KEY").await.unwrap();

        Ok(Arc::new(Self(async_openai::Client::with_config(
            async_openai::config::OpenAIConfig::new()
                .with_api_key(secret)
                .with_api_base("https://api.perplexity.ai"),
        ))))
    }

    async fn completion(
        &self,
        request: CreateCompletionRequest,
    ) -> anyhow::Result<CreateCompletionResponse> {
        Ok(self
            .0
            .chat()
            .create(request.into())
            .await
            .map(|r| r.into())?)
    }

    async fn completion_stream(
        &self,
        request: CreateCompletionRequest,
    ) -> anyhow::Result<CompletionResponseStream> {
        let mut s = self.0.chat().create_stream(request.into()).await?;

        Ok(Box::pin(async_stream::stream! {
            while let Some(item) = s.next().await {
                match item {
                    Ok(item) => yield Ok(serde_json::from_value(serde_json::to_value(item).unwrap()).unwrap()),
                    Err(e) => yield Err(e.into()),
                }
            }
        }))
    }

    async fn models(&self) -> anyhow::Result<Vec<Model>> {
        Ok(vec![
            Model {
                object: "model".to_string(),
                id: "llama-3.1-sonar-small-128k-online".to_string(),
                created: 0,
                owned_by: SupportedLlm::PerplexityAi.to_string(),
            },
            Model {
                object: "model".to_string(),
                id: "llama-3.1-sonar-large-128k-online".to_string(),
                created: 0,
                owned_by: SupportedLlm::PerplexityAi.to_string(),
            },
            Model {
                object: "model".to_string(),
                id: "llama-3.1-sonar-huge-128k-online".to_string(),
                created: 0,
                owned_by: SupportedLlm::PerplexityAi.to_string(),
            },
            Model {
                object: "model".to_string(),
                id: "llama-3.1-sonar-small-128k-chat".to_string(),
                created: 0,
                owned_by: SupportedLlm::PerplexityAi.to_string(),
            },
            Model {
                object: "model".to_string(),
                id: "llama-3.1-sonar-large-128k-chat".to_string(),
                created: 0,
                owned_by: SupportedLlm::PerplexityAi.to_string(),
            },
        ])
    }
}
