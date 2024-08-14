use std::{ops::Deref, sync::Arc};

use anthropic_vertex_ai::messages::{self, Content, Messages};
use axum::async_trait;
use futures::future::join_all;

use crate::{
    entities::{
        Choice, CompletionResponseMessage, CompletionUsage, CreateCompletionRequest,
        CreateCompletionResponse, Role,
    },
    secret_manager::SecretManagerProvider,
};

use super::{AnyLlmProvider, LlmProvider};

#[derive(Clone)]
pub struct AnthropicVertexAi(Arc<anthropic_vertex_ai::AnthropicVertexAi>);

impl Deref for AnthropicVertexAi {
    type Target = Arc<anthropic_vertex_ai::AnthropicVertexAi>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl LlmProvider for AnthropicVertexAi {
    async fn init(
        secret_manager: Arc<dyn SecretManagerProvider>,
    ) -> anyhow::Result<Arc<dyn AnyLlmProvider>> {
        let mut secrets = join_all([
            secret_manager.secret("GCLOUD_API_KEY"),
            secret_manager.secret("GCLOUD_PROJECT_ID"),
            secret_manager.secret("GCLOUD_REGION"),
        ])
        .await;

        let region = secrets.remove(2)?;
        let project = secrets.remove(1)?;
        let api_key = secrets.remove(0)?;

        Ok(Arc::new(Self(Arc::new(
            anthropic_vertex_ai::AnthropicVertexAi::builder()
                .api_key(api_key)
                .project(project)
                .region(region)
                .build()?,
        ))))
    }

    async fn completion(
        &self,
        request: CreateCompletionRequest,
    ) -> anyhow::Result<CreateCompletionResponse> {
        Ok(self
            .0
            .messages(request.into())
            .await
            .and_then(|response| match response {
                messages::CreateMessageResponse::Message(response) => {
                    Ok(CreateCompletionResponse {
                        id: response.id,
                        choices: vec![Choice {
                            index: 0,
                            logprobs: None,
                            finish_reason: None,
                            message: CompletionResponseMessage {
                                content: response.content.get(0).map(|c| match c {
                                    Content::Text { text } => text.clone(),
                                    _ => unreachable!(),
                                }),
                                tool_calls: None,
                                role: Role::Assistant,
                            },
                        }],
                        created: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("Time went backwards")
                            .as_secs() as u32,
                        model: response.model,
                        system_fingerprint: None,
                        object: "chat.completion".into(),
                        usage: Some(CompletionUsage {
                            prompt_tokens: response.usage.input_tokens.unwrap_or(0),
                            completion_tokens: response.usage.output_tokens,
                            total_tokens: response.usage.input_tokens.unwrap_or(0)
                                + response.usage.output_tokens,
                        }),
                    })
                }
                messages::CreateMessageResponse::Error { error } => {
                    Err(anyhow::anyhow!("{error:?}"))
                }
            })
            .map(|r| r.into())?)
    }
}
