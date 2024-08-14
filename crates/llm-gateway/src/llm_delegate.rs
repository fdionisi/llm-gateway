mod llm_provider;
mod supported_llm;

use std::sync::Arc;

use anyhow::bail;
use futures::future::join_all;
use llm_provider::LlmProviderMap;
pub use supported_llm::SupportedLlm;

use crate::entities::{
    CompletionResponseStream, CreateCompletionRequest, CreateCompletionResponse, ListModelResponse,
};

use super::secret_manager::SecretManagerProvider;

#[derive(Clone)]
pub struct LlmDelegate {
    secret_manager: Arc<dyn SecretManagerProvider>,
    llm_provider_map: Arc<LlmProviderMap>,
}

impl LlmDelegate {
    pub fn new(secret_manager: Arc<dyn SecretManagerProvider>) -> Self {
        Self {
            secret_manager,
            llm_provider_map: Arc::new(LlmProviderMap::default()),
        }
    }

    pub async fn completion(
        &self,
        llm: SupportedLlm,
        request: CreateCompletionRequest,
    ) -> anyhow::Result<CreateCompletionResponse> {
        if request.stream.is_some_and(|f| f) {
            bail!("streaming completions are not supported")
        }

        Ok(self
            .llm_provider_map
            .get(self.secret_manager.clone(), llm)
            .await?
            .completion(request)
            .await?)
    }

    pub async fn completion_stream(
        &self,
        llm: SupportedLlm,
        request: CreateCompletionRequest,
    ) -> anyhow::Result<CompletionResponseStream> {
        Ok(self
            .llm_provider_map
            .get(self.secret_manager.clone(), llm)
            .await?
            .completion_stream(request)
            .await?)
    }

    pub async fn models(&self) -> anyhow::Result<ListModelResponse> {
        Ok(ListModelResponse {
            data: join_all([
                self.llm_provider_map
                    .get(self.secret_manager.clone(), SupportedLlm::Anthropic)
                    .await?
                    .models(),
                self.llm_provider_map
                    .get(self.secret_manager.clone(), SupportedLlm::AnthropicVertexAi)
                    .await?
                    .models(),
                self.llm_provider_map
                    .get(self.secret_manager.clone(), SupportedLlm::OpenAi)
                    .await?
                    .models(),
                self.llm_provider_map
                    .get(self.secret_manager.clone(), SupportedLlm::PerplexityAi)
                    .await?
                    .models(),
            ])
            .await
            .into_iter()
            .filter_map(|r| r.ok())
            .flatten()
            .collect(),
            ..ListModelResponse::default()
        })
    }

    // pub async fn embeddings(
    //     &self,
    //     llm: SupportedLlm,
    //     request: CreateEmbeddingRequest,
    // ) -> Result<CreateEmbeddingResponse, OpenAIError> {
    //     match llm {
    //         SupportedLlm::OpenAi => Ok(self.openai().await?.embeddings().create(request).await?),
    //         SupportedLlm::Anthropic => todo!(),
    //         SupportedLlm::AnthropicVertexAi => todo!(),
    //     }
    // }
}
