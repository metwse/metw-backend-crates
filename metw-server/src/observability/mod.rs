//! Shared tracing and OpenTelemetry configuration for metw.cc services.
//!
//!
//! ## Cargo features
//!
//! - `otel`: Enables OTLP/gRPC trace export.
//!
//!
//! ## Initialization
//!
//! Call [`init_tracing`] once, near the beginning of the application. The
//! initialized subscriber includes:
//!
//! - An environment-based filter configured through `RUST_LOG`.
//! - A human-readable formatting layer.
//!
//!
//! ## OpenTelemetry
//!
//! When the `otel` feature is enabled, tracing spans are exported over
//! OTLP/gRPC. The collector endpoint is read from `OTEL_EXPORTER` environment
//! variable (defaults to `http://localhost:4317`). [`init_tracing_with_otel`]
//! initializes tracing with OTEL exporter.
//!
//!
//! ## HTTP request tracing
//!
//! [`trace_layer_for_http`] creates a tower `TraceLayer` that wraps HTTP
//! requests in a debug level tracing span.

#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "otel")]
mod otel;

mod init_tracing;

mod trace_layer_for_http;

pub use init_tracing::init_tracing;

#[cfg(feature = "otel")]
pub use init_tracing::init_tracing_with_otel;

pub use trace_layer_for_http::trace_layer_for_http;
