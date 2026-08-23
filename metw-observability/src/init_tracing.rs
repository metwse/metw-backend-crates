use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[cfg(feature = "otel")]
use crate::otel::otel_layer;

/// Initializes the tracing registry.
pub fn init_tracing(service_name: String) {
    let tracing_registry = tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer());

    #[cfg(feature = "otel")]
    let tracing_registry = tracing_registry.with(otel_layer(service_name));
    #[cfg(not(feature = "otel"))]
    let _ = service_name;

    tracing_registry.init();
}

#[cfg(test)]
#[tokio::test]
#[ignore]
async fn test() {
    dotenvy::dotenv().ok();

    init_tracing("test".into());
}
