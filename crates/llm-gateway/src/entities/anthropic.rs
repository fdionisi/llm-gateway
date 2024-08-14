use anthropic_vertex_ai::messages::{
    Content, CreateMessageRequest, Message, Metadata, Role, Tool, ToolChoice, ToolChoiceKind,
    ToolInputSchema,
};

use crate::entities::CompletionRequestSystemMessage;

use super::{
    CompletionNamedToolChoice, CompletionRequestAssistantMessage, CompletionRequestMessage,
    CompletionRequestUserMessage, CompletionRequestUserMessageContent, CompletionTool,
    CompletionToolChoiceOption, CompletionToolType, CreateCompletionRequest, FunctionName,
    FunctionObject, Stop,
};

impl From<CreateMessageRequest> for CreateCompletionRequest {
    fn from(request: CreateMessageRequest) -> Self {
        let mut messages = if let Some(system) = request.system {
            vec![CompletionRequestMessage::System(
                CompletionRequestSystemMessage {
                    content: system,
                    name: None,
                },
            )]
        } else {
            vec![]
        };

        messages.extend(request.messages.into_iter().map(|m| {
            match m.role {
                Role::User => CompletionRequestMessage::User(CompletionRequestUserMessage {
                    content: CompletionRequestUserMessageContent::Text(
                        m.content
                            .into_iter()
                            .map(|c| match c {
                                Content::Text { text } => text,
                                _ => String::new(), // Handle other content types if needed
                            })
                            .collect::<Vec<String>>()
                            .join("\n"),
                    ),
                    name: None,
                }),
                Role::Assistant => {
                    CompletionRequestMessage::Assistant(CompletionRequestAssistantMessage {
                        content: Some(
                            m.content
                                .into_iter()
                                .map(|c| match c {
                                    Content::Text { text } => text,
                                    _ => String::new(), // Handle other content types if needed
                                })
                                .collect::<Vec<String>>()
                                .join("\n"),
                        ),
                        name: None,
                        tool_calls: None,
                    })
                } // Add mappings for other roles if needed
            }
        }));

        CreateCompletionRequest {
            messages,
            model: request.model,
            frequency_penalty: None,
            logit_bias: None,
            logprobs: None,
            top_logprobs: None,
            max_tokens: Some(request.max_tokens),
            n: None,
            presence_penalty: None,
            response_format: None,
            seed: None,
            stop: request.stop_sequences.map(Stop::StringArray),
            stream: None,
            stream_options: None,
            temperature: request.temperature,
            top_p: request.top_p,
            tools: request.tools.map(|tools| {
                tools
                    .into_iter()
                    .map(|tool| CompletionTool {
                        kind: CompletionToolType::Function,
                        function: FunctionObject {
                            name: tool.name,
                            description: tool.description,
                            parameters: Some(tool.input_schema.properties.unwrap_or_default()),
                        },
                    })
                    .collect()
            }),
            tool_choice: request.tool_choice.map(|choice| match choice.kind {
                ToolChoiceKind::Auto => CompletionToolChoiceOption::Auto,
                ToolChoiceKind::Any => CompletionToolChoiceOption::Required,
                ToolChoiceKind::Tool => {
                    CompletionToolChoiceOption::Named(CompletionNamedToolChoice {
                        kind: CompletionToolType::Function,
                        function: FunctionName {
                            name: String::new(),
                        },
                    })
                }
            }),
            parallel_tool_calls: None,
            user: request.metadata.and_then(|m| m.user_id),
        }
    }
}

impl Into<CreateMessageRequest> for CreateCompletionRequest {
    fn into(self) -> CreateMessageRequest {
        let system = self.messages.first().and_then(|m| {
            if let CompletionRequestMessage::System(sys) = m {
                Some(sys.content.clone())
            } else {
                None
            }
        });

        let messages = self
            .messages
            .into_iter()
            .filter_map(|m| match m {
                CompletionRequestMessage::User(user) => Some(Message {
                    role: Role::User,
                    content: vec![Content::Text {
                        text: match user.content {
                            CompletionRequestUserMessageContent::Text(text) => text,
                            CompletionRequestUserMessageContent::Array(_) => String::new(),
                        },
                    }],
                }),
                CompletionRequestMessage::Assistant(assistant) => Some(Message {
                    role: Role::Assistant,
                    content: vec![Content::Text {
                        text: assistant.content.unwrap_or_default(),
                    }],
                }),
                _ => None,
            })
            .collect();

        CreateMessageRequest {
            model: self.model,
            messages,
            max_tokens: self.max_tokens.unwrap_or(4096),
            metadata: self.user.map(|user_id| Metadata {
                user_id: Some(user_id),
            }),
            stop_sequences: match self.stop {
                Some(Stop::StringArray(arr)) => Some(arr),
                Some(Stop::String(s)) => Some(vec![s]),
                None => None,
            },
            system,
            temperature: self.temperature,
            tool_choice: self.tool_choice.map(|choice| match choice {
                CompletionToolChoiceOption::Auto => ToolChoice {
                    kind: ToolChoiceKind::Auto,
                },
                CompletionToolChoiceOption::Required => ToolChoice {
                    kind: ToolChoiceKind::Any,
                },
                CompletionToolChoiceOption::Named(_) => ToolChoice {
                    kind: ToolChoiceKind::Tool,
                },
                CompletionToolChoiceOption::None => ToolChoice {
                    kind: ToolChoiceKind::Auto,
                },
            }),
            tools: self.tools.map(|tools| {
                tools
                    .into_iter()
                    .map(|tool| Tool {
                        description: tool.function.description,
                        name: tool.function.name,
                        input_schema: ToolInputSchema {
                            type_: "object".to_string(),
                            properties: tool.function.parameters,
                        },
                    })
                    .collect()
            }),
            top_p: self.top_p,
            top_k: None,
        }
    }
}
