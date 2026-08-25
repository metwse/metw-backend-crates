use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::SdkTracerProvider;
use std::env;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::registry::LookupSpan;

pub fn otel_layer<S: tracing::Subscriber + for<'span> LookupSpan<'span>>(
    service_name: String,
) -> (
    OpenTelemetryLayer<S, opentelemetry_sdk::trace::Tracer>,
    SdkTracerProvider,
) {
    let otlp_exporter_endpoint = env::var("OTEL_EXPORTER").unwrap_or_else(|_| {
        println!(
            "OTEL_EXPORTER environment variable is not set, defaulting to http://localhost:4317"
        );

        "http://localhost:4317".to_string()
    });

    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(otlp_exporter_endpoint)
        .build()
        .unwrap();

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(otlp_exporter)
        .with_resource(
            opentelemetry_sdk::Resource::builder()
                .with_service_name(service_name.clone())
                .build(),
        )
        .build();

    let tracer = provider.tracer(service_name);

    (OpenTelemetryLayer::new(tracer), provider)
}
