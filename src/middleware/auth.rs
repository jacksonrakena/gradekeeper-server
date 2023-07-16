use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;

pub async fn check_authorization<B>(request: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let response = next.run(request).await;

    Ok(response)
}