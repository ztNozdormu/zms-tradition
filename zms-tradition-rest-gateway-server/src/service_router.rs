use std::sync::Arc;
use std::time::Duration;

use axum::{
    body::Bytes,
    http::header,
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::{
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    LatencyUnit, ServiceBuilderExt,
};
use tracing::{info, instrument};

use crate::gw3data_client::query_picker_symbols;

const QUERY_PICKER_SYMBOLS: &str = "/api/zms-tradition/query_picker_symbols";

pub async fn root() -> String {
    format!(
        "For Query picker bot symbols API, use POST with Path {}",
        QUERY_PICKER_SYMBOLS
    )
}

#[instrument]
pub async fn router() -> Router {
    let sensitive_headers: Arc<[_]> = vec![header::AUTHORIZATION].into();

    let middleware = ServiceBuilder::new()
        .sensitive_request_headers(sensitive_headers.clone())
        .layer(
            TraceLayer::new_for_http()
                .on_body_chunk(|chunk: &Bytes, latency: Duration, _: &tracing::Span| {
                    tracing::trace!(size_bytes = chunk.len(), latency = ?latency, "sending body chunk")
                })
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_response(DefaultOnResponse::new().include_headers(true).latency_unit(LatencyUnit::Micros)),
        )
        .sensitive_response_headers(sensitive_headers)
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .compression();
    info!("Creating router");
    Router::new()
        .route("/", get(root).post(root))
        .route(QUERY_PICKER_SYMBOLS, post(query_picker_symbols))
        .layer(middleware)
}
