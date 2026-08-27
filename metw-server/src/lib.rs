//! # metw-server
//!
//! Serve axum application with good defaults.
//!
//!
//! ## Cargo features
//!
//! - `otel`: Enables OTLP/gRPC trace export. See [`observability`].
//! - `allow-all-cors`: Allows CORS for all hosts.

use axum::Router;
use std::{env, net::SocketAddr};
use tokio::signal;

pub mod observability;

/// Serve the application.
pub async fn serve(app: Router, service_name: String) {
    #[cfg(feature = "otel")]
    let tracer_provider = observability::init_tracing_with_otel(service_name);
    #[cfg(not(feature = "otel"))]
    {
        observability::init_tracing();

        let _ = service_name;
    }

    let sock_addr: SocketAddr = env::var("HOST")
        .unwrap_or_else(|_| {
            println!("HOST environment variable is not set, defaulting to 127.0.0.1:3781");

            "127.0.0.1:3781".to_string()
        })
        .parse()
        .unwrap();

    let listener = tokio::net::TcpListener::bind(&sock_addr).await.unwrap();

    #[cfg(feature = "allow-all-cors")]
    let app = {
        use tower_http::cors::{AllowHeaders, AllowMethods, Any, CorsLayer};

        let cors = CorsLayer::new()
            .allow_methods(AllowMethods::any())
            .allow_origin(Any)
            .allow_headers(AllowHeaders::any());

        app.layer(cors).layer(observability::trace_layer_for_http())
    };
    #[cfg(not(feature = "allow-all-cors"))]
    let app = app.layer(observability::trace_layer_for_http());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    #[cfg(feature = "otel")]
    tracer_provider.shutdown().unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
