use std::net::SocketAddr;
use std::time::Duration;

use serde::Deserialize;
use toolcraft_config::load_settings;

use crate::input::network::NetworkSourceConfig;
use crate::runtime::ExternalRuntimeConfig;

#[derive(Debug, Deserialize)]
pub struct ExternalRuntimeSettings {
    #[serde(default = "default_tick_interval_ms")]
    pub tick_interval_ms: u64,
    #[serde(default)]
    pub network: NetworkSettings,
}

impl ExternalRuntimeSettings {
    pub fn load(config_path: &str) -> toolcraft_config::Result<Self> {
        load_settings(config_path)
    }
}

impl From<ExternalRuntimeSettings> for ExternalRuntimeConfig {
    fn from(settings: ExternalRuntimeSettings) -> Self {
        Self {
            tick_interval: Duration::from_millis(settings.tick_interval_ms),
            network: settings.network.into_source_config(),
        }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct NetworkSettings {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub local_addr: Option<SocketAddr>,
    #[serde(default)]
    pub remote_addr: Option<SocketAddr>,
    #[serde(default)]
    pub reconnect_interval_ms: Option<u64>,
}

impl NetworkSettings {
    fn into_source_config(self) -> Option<NetworkSourceConfig> {
        if !self.enabled {
            return None;
        }

        let mut config = NetworkSourceConfig::new(self.local_addr?, self.remote_addr?);

        if let Some(ms) = self.reconnect_interval_ms {
            config.reconnect_interval = Duration::from_millis(ms);
        }

        Some(config)
    }
}

fn default_tick_interval_ms() -> u64 {
    16
}
