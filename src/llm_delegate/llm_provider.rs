mod anthropic;
mod anthropic_vertexai;
mod openai;
mod perplexityai;

use std::{collections::HashMap, sync::Arc};

use anthropic::Anthropic;
use anyhow::Result;
use axum::async_trait;
use perplexityai::PerplexityAi;
use tokio::sync::Mutex;

use crate::{
    entities::{
        CompletionResponseStream, CreateCompletionRequest, CreateCompletionResponse, Model,
    },
    llm_delegate::SupportedLlm,
    secret_manager::SecretManagerProvider,
};

use anthropic_vertexai::AnthropicVertexAi;
use openai::OpenAi;

#[async_trait]
pub trait AnyLlmProvider: Send + Sync {
    async fn completion(
        &self,
        request: CreateCompletionRequest,
    ) -> anyhow::Result<CreateCompletionResponse>;

    async fn completion_stream(
        &self,
        request: CreateCompletionRequest,
    ) -> anyhow::Result<CompletionResponseStream>;

    async fn models(&self) -> anyhow::Result<Vec<Model>>;
}

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn init(
        secret_manager: Arc<dyn SecretManagerProvider>,
    ) -> anyhow::Result<Arc<dyn AnyLlmProvider>>;

    async fn completion(
        &self,
        request: CreateCompletionRequest,
    ) -> anyhow::Result<CreateCompletionResponse>;

    async fn completion_stream(
        &self,
        _request: CreateCompletionRequest,
    ) -> anyhow::Result<CompletionResponseStream> {
        todo!()
    }

    async fn models(&self) -> anyhow::Result<Vec<Model>>;
}

#[async_trait]
impl<T> AnyLlmProvider for T
where
    T: LlmProvider + 'static,
{
    async fn completion(
        &self,
        request: CreateCompletionRequest,
    ) -> anyhow::Result<CreateCompletionResponse> {
        Ok(self.completion(request).await?)
    }

    async fn completion_stream(
        &self,
        request: CreateCompletionRequest,
    ) -> anyhow::Result<CompletionResponseStream> {
        Ok(self.completion_stream(request).await?)
    }

    async fn models(&self) -> anyhow::Result<Vec<Model>> {
        Ok(self.models().await?)
    }
}

#[derive(Default)]
pub struct LlmProviderMap(Mutex<HashMap<SupportedLlm, Arc<dyn AnyLlmProvider>>>);

impl LlmProviderMap {
    pub async fn get(
        &self,
        secret_manager: Arc<dyn SecretManagerProvider>,
        llm: SupportedLlm,
    ) -> Result<Arc<dyn AnyLlmProvider>> {
        let mut self_guard = self.0.lock().await;
        if !self_guard.contains_key(&llm) {
            self_guard.insert(
                llm,
                match llm {
                    SupportedLlm::OpenAi => OpenAi::init(secret_manager).await?,
                    SupportedLlm::Anthropic => Anthropic::init(secret_manager).await?,
                    SupportedLlm::AnthropicVertexAi => {
                        AnthropicVertexAi::init(secret_manager).await?
                    }
                    SupportedLlm::PerplexityAi => PerplexityAi::init(secret_manager).await?,
                },
            );
        }

        Ok(self_guard.get(&llm).unwrap().to_owned())
    }
}
