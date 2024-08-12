use std::{convert::Infallible, sync::Arc};

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};

use super::auth_provider::AuthProvider;

pub async fn auth_middleware(
    State(auth_provider): State<Arc<dyn AuthProvider + Send + Sync>>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    mut request: Request,
    next: Next,
) -> Result<Response, Infallible> {
    let claims = match auth_provider.verify(authorization.token()).await {
        Ok(claims) => claims,
        Err(err) => return Ok(err.into_response()),
    };

    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}
