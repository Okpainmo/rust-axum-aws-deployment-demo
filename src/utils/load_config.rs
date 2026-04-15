//! # Configuration Management
//!
//! This module handles loading and validating the application configuration from
//! multiple sources: base TOML files, environment-specific overrides, local
//! overrides, and environment variables.

use anyhow::{Context, Result};
use config::{Config, Environment, File};
use serde::Deserialize;
use std::fmt;

/// Application-specific metadata section.
#[derive(Debug, Deserialize)]
pub struct AppSection {
    /// The name of the application.
    pub name: String,
    /// The current environment (e.g., development, production).
    pub environment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ObservabilitySection {
    pub enable_tracing: bool,
    pub enable_metrics: bool,
}

#[derive(Debug, Deserialize)]
pub struct ServerSection {
    pub host: String,
    pub port: u16,
    pub request_timeout_secs: u64,
}

/// Root configuration structure containing all application settings.
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub app: AppSection,
    pub server: Option<ServerSection>,
    pub observability: ObservabilitySection,
}

/// Loads the application configuration.
///
/// Order of precedence (highest to lowest):
/// 1. Environment variables (prefixed with `APP__`) - overrides every other configuration setup
/// 2. `config/local.toml` - overrides `config/{APP__ENV}.toml` and `config/base.toml`
/// 3. `config/{APP__ENV}.toml` - overrides `config/base.toml`
/// 4. `config/base.toml` - default values
pub fn load_config() -> Result<AppConfig> {
    // Determine environment
    let env = std::env::var("APP__ENV").context("APP__ENV environment variable is not set! Please set it to one of 'development', 'production', etc.")?;

    // Build configuration
    let builder = Config::builder()
        // Base config is required
        .add_source(File::with_name("config/base").required(true))
        // Environment-specific overrides (optional)
        .add_source(File::with_name(&format!("config/{}", env)).required(false))
        // Local overrides (optional, for dev machines)
        .add_source(File::with_name("config/local").required(false))
        // Environment variable overrides
        .add_source(
            Environment::default()
                .separator("__") // maps APP__SECTION__FIELD → section.field
                .prefix("APP") // all vars must start with APP__
                .try_parsing(true), // parse numbers/booleans automatically
        );

    /**************** EXPLAINING THE MAPPING RULE FOR THE [ABOVE] FINAL ENV OVERRIDES ****************
    # Mapping Rule (exact)

    APP__<SECTION>__<FIELD>=value - E.g. APP__SERVER__PORT=9000

    Lowercase / uppercase differences are normalized(handled without manual intervention).

    So this TOML:

    [server]
    port = 8080

    will be overridden by:

    APP__SERVER__PORT=9000

    If the names don’t align, nothing happens.

    Example (❌ no override):

    SERVER_PORT=9000

    This does nothing unless you explicitly read it in code.

    **************** EXPLAINING THE MAPPING RULE FOR THE [ABOVE] FINAL ENV OVERRIDES ****************/

    builder
        .build()
        .context("Failed to build config")?
        .try_deserialize()
        .context("Invalid config shape")
}

#[derive(Debug)]
pub enum ConfigError {
    MissingAppName,
    InvalidServerPort,
    MissingServerSection,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingAppName => write!(f, "app.name cannot be empty"),
            ConfigError::InvalidServerPort => write!(f, "server.port cannot be 0"),
            ConfigError::MissingServerSection => write!(f, "server section is missing"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl AppConfig {
    pub fn validate(&self) -> std::result::Result<(), ConfigError> {
        // Check app name
        if self.app.name.trim().is_empty() {
            return Err(ConfigError::MissingAppName);
        }

        // Check server
        let server = self
            .server
            .as_ref()
            .ok_or(ConfigError::MissingServerSection)?;

        if server.port == 0 {
            return Err(ConfigError::InvalidServerPort);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_app_section() -> AppSection {
        AppSection {
            name: "Test App".to_string(),
            environment: Some("development".to_string()),
        }
    }

    #[test]
    fn test_validate_valid_config() {
        let config = AppConfig {
            app: valid_app_section(),
            observability: ObservabilitySection {
                enable_tracing: true,
                enable_metrics: true,
            },
            server: Some(ServerSection {
                host: "127.0.0.1".to_string(),
                port: 8080,
                request_timeout_secs: 60,
            }),
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_missing_app_name() {
        let mut config = AppConfig {
            app: valid_app_section(),
            observability: ObservabilitySection {
                enable_tracing: false,
                enable_metrics: false,
            },
            server: Some(ServerSection {
                host: "127.0.0.1".to_string(),
                port: 8080,
                request_timeout_secs: 60,
            }),
        };

        config.app.name = "".to_string();

        let result = config.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "app.name cannot be empty");
    }

    #[test]
    fn test_validate_invalid_port() {
        let config = AppConfig {
            app: valid_app_section(),
            observability: ObservabilitySection {
                enable_tracing: false,
                enable_metrics: false,
            },
            server: Some(ServerSection {
                host: "127.0.0.1".to_string(),
                port: 0,
                request_timeout_secs: 60,
            }),
           
        };

        let result = config.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "server.port cannot be 0");
    }

    #[test]
    fn test_validate_missing_server_section() {
        let config = AppConfig {
            app: valid_app_section(),
            observability: ObservabilitySection {
                enable_tracing: false,
                enable_metrics: false,
            },
            server: None,
        };

        let result = config.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "server section is missing");
    }
}