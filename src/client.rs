//! PidginHost API Client
//!
//! Simple wrapper around the generated SDK.
//!
//! ```rust
//! let client = pidginhost_sdk::PidginHost::new("your-api-token");
//! let servers = pidginhost_sdk::apis::cloud_api::cloud_servers_list(&client.config).await?;
//! ```

use crate::apis::configuration::{ApiKey, Configuration};

/// PidginHost API client.
pub struct PidginHost {
    pub config: Configuration,
}

impl PidginHost {
    /// Create a new client with the given API token.
    pub fn new(token: &str) -> Self {
        let mut config = Configuration::new();
        config.api_key = Some(ApiKey {
            key: token.to_string(),
            prefix: Some("Token".to_string()),
        });
        Self { config }
    }

    /// Create a new client with a custom host URL.
    pub fn with_host(token: &str, host: &str) -> Self {
        let mut config = Configuration::new();
        config.base_path = host.to_string();
        config.api_key = Some(ApiKey {
            key: token.to_string(),
            prefix: Some("Token".to_string()),
        });
        Self { config }
    }
}
