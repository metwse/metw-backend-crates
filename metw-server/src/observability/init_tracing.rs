use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[cfg(feature = "otel")]
use opentelemetry_sdk::trace::SdkTracerProvider;

#[cfg(feature = "otel")]
use super::otel::otel_layer;

/// Initializes the tracing registry.
pub fn init_tracing() {
    let tracing_registry = tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer());

    tracing_registry.init();
}

/// Initializes the tracing registry, with an OTEL exporter.
#[cfg(feature = "otel")]
pub fn init_tracing_with_otel(service_name: String) -> SdkTracerProvider {
    let (tracer, provider) = otel_layer(service_name);

    let tracing_registry = tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .with(tracer);

    tracing_registry.init();

    provider
}

#[cfg(all(test, not(feature = "otel")))]
#[test]
fn test() {
    init_tracing();
}

#[cfg(all(test, feature = "otel"))]
#[tokio::test]
#[ignore]
async fn test_with_otel() {
    dotenvy::dotenv().ok();

    init_tracing_with_otel("test".into());
}
