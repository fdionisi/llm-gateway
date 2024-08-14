use std::sync::Arc;

use axum::async_trait;
use futures::StreamExt;

use crate::{
    entities::{
        CompletionResponseStream, CreateCompletionRequest, CreateCompletionResponse,
        ListModelResponse, Model,
    },
    secret_manager::SecretManagerProvider,
};

use super::{AnyLlmProvider, LlmProvider};

pub struct OpenAi(async_openai::Client<async_openai::config::OpenAIConfig>);

#[async_trait]
impl LlmProvider for OpenAi {
    async fn init(
        secret_manager: Arc<dyn SecretManagerProvider>,
    ) -> anyhow::Result<Arc<dyn AnyLlmProvider>> {
        let secret = secret_manager.secret("OPENAI_API_KEY").await.unwrap();

        Ok(Arc::new(Self(async_openai::Client::with_config(
            async_openai::config::OpenAIConfig::new().with_api_key(secret),
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
        let models = self.0.models().list().await?;
        Ok(serde_json::from_value::<ListModelResponse>(serde_json::to_value(models)?)?.data)
    }
}
