use std::hash::Hash;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use headers::{Header, HeaderName, HeaderValue};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum SupportedLlm {
    OpenAi,
    Anthropic,
    AnthropicVertexAi,
    PerplexityAi,
}

impl SupportedLlm {
    fn as_str(&self) -> &'static str {
        match self {
            Self::OpenAi => "openai",
            Self::Anthropic => "anthropic",
            Self::AnthropicVertexAi => "vertexai.anthropic",
            Self::PerplexityAi => "perplexityai",
        }
    }
}

impl ToString for SupportedLlm {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl Hash for SupportedLlm {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl TryFrom<&str> for SupportedLlm {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "openai" => Ok(Self::OpenAi),
            "anthropic" => Ok(Self::Anthropic),
            "vertexai.anthropic" => Ok(Self::AnthropicVertexAi),
            "perplexityai" => Ok(Self::PerplexityAi),
            _ => Err(anyhow::anyhow!("Unsupported LLM provider")),
        }
    }
}

impl Header for SupportedLlm {
    fn name() -> &'static HeaderName {
        static NAME: HeaderName = HeaderName::from_static("x-llm-provider");
        &NAME
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        let raw = values
            .next()
            .ok_or_else(headers::Error::invalid)?
            .to_str()
            .map_err(|_| headers::Error::invalid())?;

        Self::try_from(raw).map_err(|_| headers::Error::invalid())
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        values.extend(std::iter::once(HeaderValue::from_static(self.as_str())));
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for SupportedLlm {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(SupportedLlm::name())
            .ok_or(StatusCode::BAD_REQUEST)?;

        let llm = SupportedLlm::decode(&mut std::iter::once(header))
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        Ok(llm)
    }
}
