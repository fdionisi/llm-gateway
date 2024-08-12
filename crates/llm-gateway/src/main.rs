mod app_state;
mod auth;
mod llm_delegate;
mod secret_manager;

use app_state::AppState;
use async_openai::types::{
    CreateChatCompletionRequest, CreateChatCompletionResponse, CreateEmbeddingRequest,
    CreateEmbeddingResponse,
};
use auth::auth_middleware;
use axum::{extract::State, middleware, routing::post, Json, Router};
use llm_delegate::{LlmDelegate, SupportedLlm};
use std::convert::Infallible;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "llm-gateway=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = app();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    let app_state = AppState::new(
        LlmDelegate::new(secret_manager::EnvSecretManagerProvider::new()),
        auth::Auth0AuthProvider::builder().build(),
    );

    Router::new()
        .route("/v1/chat/completions", post(chat_completions))
        .route("/v1/embeddings", post(embeddings))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}

async fn embeddings(
    State(llm_delegate): State<LlmDelegate>,
    Json(request): Json<CreateEmbeddingRequest>,
) -> Result<Json<CreateEmbeddingResponse>, Infallible> {
    Ok(Json(
        llm_delegate
            .embeddings(SupportedLlm::OpenAi, request)
            .await
            .unwrap(),
    ))
}

async fn chat_completions(
    State(llm_delegate): State<LlmDelegate>,
    Json(request): Json<CreateChatCompletionRequest>,
) -> Result<Json<CreateChatCompletionResponse>, Infallible> {
    Ok(Json(
        llm_delegate
            .completion(SupportedLlm::OpenAi, request)
            .await
            .unwrap(),
    ))
}

#[cfg(test)]
mod tests {
    // use eventsource_stream::Eventsource;
    // use tokio::net::TcpListener;

    // use super::*;

    // #[tokio::test]
    // async fn integration_test() {
    //     // A helper function that spawns our application in the background
    //     async fn spawn_app(host: impl Into<String>) -> String {
    //         let host = host.into();
    //         // Bind to localhost at the port 0, which will let the OS assign an available port to us
    //         let listener = TcpListener::bind(format!("{}:0", host)).await.unwrap();
    //         // Retrieve the port assigned to us by the OS
    //         let port = listener.local_addr().unwrap().port();
    //         tokio::spawn(async {
    //             axum::serve(listener, app()).await.unwrap();
    //         });
    //         // Returns address (e.g. http://127.0.0.1{random_port})
    //         format!("http://{}:{}", host, port)
    //     }
    //     let listening_url = spawn_app("127.0.0.1").await;

    //     let mut event_stream = reqwest::Client::new()
    //         .get(format!("{}/sse", listening_url))
    //         .header("User-Agent", "integration_test")
    //         .send()
    //         .await
    //         .unwrap()
    //         .bytes_stream()
    //         .eventsource()
    //         .take(1);

    //     let mut event_data: Vec<String> = vec![];
    //     while let Some(event) = event_stream.next().await {
    //         match event {
    //             Ok(event) => {
    //                 // break the loop at the end of SSE stream
    //                 if event.data == "[DONE]" {
    //                     break;
    //                 }

    //                 event_data.push(event.data);
    //             }
    //             Err(_) => {
    //                 panic!("Error in event stream");
    //             }
    //         }
    //     }

    //     assert!(event_data[0] == "hi!");
    // }
}
