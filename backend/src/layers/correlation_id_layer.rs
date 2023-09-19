use axum::http::{HeaderValue, Request, StatusCode};
use axum::response::Response;
use axum::middleware::Next;
use uuid::Uuid;

pub struct CorrelationId(pub String);

/// Middleware that attaches a correlation id to the request/response (if error occurred)
pub async fn correlation_id_extension<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let correlation_id = Uuid::new_v4().to_string();
    req
        .extensions_mut()
        .insert(CorrelationId(correlation_id.clone()));
    let mut response = next.run(req).await;
    if response.status().is_server_error() {
        response
            .headers_mut()
            .insert("x-correlation-id", correlation_id
                .parse()
                .unwrap_or(HeaderValue::from(0)));
    }

    Ok(response)
}