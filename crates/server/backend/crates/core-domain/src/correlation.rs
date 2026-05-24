use axum::body::Body;
use axum::http::{HeaderName, HeaderValue, Request};
use axum::middleware::Next;
use axum::response::Response;
use uuid::Uuid;

pub const CORRELATION_HEADER: &str = "x-correlation-id";

pub async fn correlation_middleware(mut request: Request<Body>, next: Next) -> Response {
    let header = HeaderName::from_static(CORRELATION_HEADER);
    let correlation_id = request
        .headers()
        .get(&header)
        .and_then(|value| value.to_str().ok())
        .filter(|value| !value.trim().is_empty())
        .map(ToString::to_string)
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let header_value = HeaderValue::from_str(&correlation_id)
        .unwrap_or_else(|_| HeaderValue::from_static("invalid-correlation-id"));
    request.headers_mut().insert(header.clone(), header_value.clone());

    let mut response = next.run(request).await;
    response.headers_mut().insert(header, header_value);
    response
}
