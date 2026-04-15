//! # Chat Auth Server Binary
//!
//! The entry point for the authentication server. This binary handles:
//! - Environment variable loading.
//! - Logging initialization.
//! - Configuration validation.
//! - Server binding and execution.

use rust_axum_aws_deployment_demo::utils::load_config::load_config;
use rust_axum_aws_deployment_demo::utils::load_env::load_env;
use rust_axum_aws_deployment_demo::{AppState, create_app};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{error, info};
use tracing_subscriber::fmt::time::SystemTime;

/// Initializes the global tracing subscriber with JSON formatting.
fn initialize_logging() {
    tracing_subscriber::fmt()
        .json()
        .with_timer(SystemTime)
        .with_level(true)
        .init();
}   

#[tokio::main]
async fn main() {
    load_env();
    initialize_logging();

    let app_config = load_config();

    // println!("{:?}", app_config);

    let clean_config = match app_config {
        Ok(config) => {
            if let Err(e) = config.validate() {
                let error = format!(
                    "SERVER START-UP ERROR: FAILED TO LOAD SERVER CONFIGURATIONS, {}",
                    e
                );
                error!("{}", error);
                std::process::exit(1);
            }

            config
        }
        Err(e) => {
            let error = format!(
                "SERVER START-UP ERROR: FAILED TO LOAD SERVER CONFIGURATIONS, {}",
                e
            );
            error!("{}", error);
            std::process::exit(1);
        }
    };


    let state = AppState {
        config: Arc::new(clean_config),
    };

    let app = create_app(state.clone());

    let host = state
        .config
        .server
        .as_ref()
        .map(|s| s.host.as_str())
        .unwrap_or("0.0.0.0");

    let port = state.config.server.as_ref().map(|s| s.port).unwrap_or(8000);

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid server address");

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => {
            print!(
                "
                .................................................
                Environment: {}
                Status: server started successfully
                .................................................

                Server running on http://{}
                ",
                state.config.app.environment.as_deref().unwrap_or("unknown"),
                addr
            );
            listener
        }
        Err(e) => {
            error!("SERVER INITIALIZATION ERROR: {}!", e);
            std::process::exit(1);
        }
    };

    let server_result = axum::serve(listener, app).await;

    match server_result {
        Ok(_) => {
            info!("Graceful server shutdown!");
        }
        Err(e) => {
            error!("SERVER SHUTDOWN ERROR: {}!", e);
        }
    }
}