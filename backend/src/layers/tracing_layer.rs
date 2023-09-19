use std::time::Duration;
use axum::Error;
use axum::extract::MatchedPath;
use bytes::Bytes;
use http::{HeaderMap, Request, Response};
use http_body::combinators::UnsyncBoxBody;
use hyper::Body;
use tower_http::classify::{ServerErrorsAsFailures, ServerErrorsFailureClass, SharedClassifier};
use tower_http::trace::TraceLayer;
use tracing::{debug_span, Span};
use crate::layers::correlation_id_layer::CorrelationId;

pub fn create_tracing_layer()
    -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>,
        impl Fn(&Request<Body>) -> Span + Clone, // make_span_with
        impl Fn(&Request<Body>, &Span) + Clone, // on_request
        impl Fn(&Response<UnsyncBoxBody<Bytes, Error>>, Duration, &Span) + Clone, // on_response
        impl Fn(&Bytes, Duration, &Span) + Clone, // on_body_chunk
        impl Fn(Option<&HeaderMap>, Duration, &Span) + Clone, // on_eos
        impl Fn(ServerErrorsFailureClass, Duration, &Span) + Clone> // on_failure
{
    TraceLayer::new_for_http()
        .make_span_with(|request: &Request<_>| {
            // Log the matched route's path (with placeholders not filled in).
            // Use request.uri() or OriginalUri if you want the real path.
            let matched_path = request
                .extensions()
                .get::<MatchedPath>()
                .map(MatchedPath::as_str);

            let correlation_id = request.extensions()
                .get::<CorrelationId>();

            debug_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    correlation_id = correlation_id.unwrap_or(&CorrelationId("None".to_string())).0
                )
        })
        .on_request(|request: &Request<Body>, _span: &Span| {
            tracing::debug!("started {} {}", request.method(), request.uri().path())
        })
        .on_response(|_response: &Response<UnsyncBoxBody<Bytes, Error>>, latency: Duration, _span: &Span| {
            tracing::debug!("response generated in {:?}", latency)
        })
        .on_body_chunk(|chunk: &Bytes, _latency: Duration, _span: &Span| {
            match String::from_utf8(chunk.to_vec()) {
                Ok(string) => tracing::debug!("sending {}", string),
                Err(_) => tracing::debug!("sending {} bytes", chunk.len())
            }
        })
        .on_eos(|_trailers: Option<&HeaderMap>, stream_duration: Duration, _span: &Span| {
            tracing::debug!("stream closed after {:?}", stream_duration)
        })
        .on_failure(|_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {})
}

