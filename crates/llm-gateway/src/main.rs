mod app_state;
mod entities;
mod llm_delegate;
mod secret_manager;

use app_state::AppState;
use auth::{auth_middleware, cached_jwk_set::CachedJwkSet};
use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    response::{
        sse::{Event, Sse},
        IntoResponse, Response,
    },
    routing::{get, post},
    Json, Router,
};
use axum_extra::TypedHeader;
use entities::CreateCompletionRequest;
use llm_delegate::{LlmDelegate, SupportedLlm};
use std::{convert::Infallible, sync::Arc, time::Duration};
use tokio_stream::StreamExt;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "llm-gateway=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = app()?;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await?;

    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

fn app() -> anyhow::Result<Router> {
    let audicence = std::env::var("AUTH0_AUDIENCE")?;
    let app_state = AppState::new(
        LlmDelegate::new(secret_manager::EnvSecretManagerProvider::new()),
        Arc::new(
            CachedJwkSet::builder()
                .issuer(std::env::var("AUTH0_ISSUER_BASE_URL")?)
                .duration(Duration::from_secs(60 * 60 * 24))
                .validator(Arc::new(move |mut validation| {
                    validation.set_audience(&[&audicence]);
                    validation.to_owned()
                }))
                .build()?,
        ),
    );

    Ok(Router::new()
        .route("/v1/chat/completions", post(completions))
        .route("/v1/embeddings", post(embeddings))
        .route("/v1/models", get(models))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state))
}

async fn models(State(llm_delegate): State<LlmDelegate>) -> Response {
    Json(llm_delegate.models().await.unwrap()).into_response()
}

async fn embeddings() -> Response {
    (StatusCode::NOT_IMPLEMENTED, "".to_string()).into_response()
}

async fn completions(
    State(llm_delegate): State<LlmDelegate>,
    TypedHeader(llm): TypedHeader<SupportedLlm>,
    Json(request): Json<CreateCompletionRequest>,
) -> Response {
    if request.stream.is_some_and(|f| f) {
        return completions_stream(State(llm_delegate), TypedHeader(llm), Json(request)).await;
    } else {
        return completions_http(State(llm_delegate), TypedHeader(llm), Json(request)).await;
    }
}

async fn completions_http(
    State(llm_delegate): State<LlmDelegate>,
    TypedHeader(llm): TypedHeader<SupportedLlm>,
    Json(request): Json<CreateCompletionRequest>,
) -> Response {
    Json(llm_delegate.completion(llm, request).await.unwrap()).into_response()
}

async fn completions_stream(
    State(llm_delegate): State<LlmDelegate>,
    TypedHeader(llm): TypedHeader<SupportedLlm>,
    Json(request): Json<CreateCompletionRequest>,
) -> Response {
    let stream = llm_delegate.completion_stream(llm, request).await.unwrap();
    let stream = stream.map(|item| {
        Ok::<Event, Infallible>(
            Event::default().data(&serde_json::to_string(&item.unwrap()).unwrap()),
        )
    });

    Sse::new(stream)
        .keep_alive(
            axum::response::sse::KeepAlive::new()
                .interval(Duration::from_secs(1))
                .text("keep-alive-text"),
        )
        .into_response()
}
