use axum::extract::MatchedPath;
use http::Request;
use tower_http::trace::{HttpMakeClassifier, MakeSpan, TraceLayer};
use tracing::Span;

#[derive(Clone, Copy)]
pub struct MakeSpanImpl;

impl<B> MakeSpan<B> for MakeSpanImpl {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        if cfg!(feature = "otel") {
            let matched_path = request
                .extensions()
                .get::<MatchedPath>()
                .map(|matched_path| matched_path.as_str());

            if let Some(matched_path) = matched_path {
                let method = request.method().as_str();

                return tracing::debug_span!(
                    "request",
                    "otel.name" = format!("{method} {matched_path}")
                );
            }
        } else {
            let _ = request;
        }

        tracing::debug_span!("request")
    }
}

/// Initiazlies trace layer for tower_http.
pub fn trace_layer_for_http() -> TraceLayer<HttpMakeClassifier, MakeSpanImpl> {
    TraceLayer::new_for_http().make_span_with(MakeSpanImpl)
}

#[cfg(test)]
#[test]
fn test() {
    use axum::Router;

    let _ = Router::<()>::new().layer(trace_layer_for_http());
}
