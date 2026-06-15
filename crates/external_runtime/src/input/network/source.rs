use std::net::SocketAddr;
use std::time::Duration;

use network::connection::{NetworkClient, NetworkClientConfig, NetworkClientEvent};
use network::router::{TocRouter, demo_toc_router};

use crate::manager::ExternalRuntimeManager;

#[derive(Debug, Clone, Copy)]
pub struct NetworkSourceConfig {
    pub local_addr: SocketAddr,
    pub remote_addr: SocketAddr,
    pub reconnect_interval: Duration,
}

impl NetworkSourceConfig {
    pub fn new(local_addr: SocketAddr, remote_addr: SocketAddr) -> Self {
        Self {
            local_addr,
            remote_addr,
            reconnect_interval: NetworkClientConfig::default().reconnect_interval,
        }
    }
}

pub struct NetworkSource {
    client: NetworkClient,
    router: TocRouter,
}

impl NetworkSource {
    pub async fn connect(config: Option<NetworkSourceConfig>) -> Option<Self> {
        let config = config?;
        let client_config = NetworkClientConfig {
            reconnect_interval: config.reconnect_interval,
        };
        let client = NetworkClient::connect_with_config(
            config.local_addr,
            config.remote_addr,
            client_config,
        )
        .await
        .ok()?;

        Some(Self {
            client,
            router: demo_toc_router(),
        })
    }

    pub async fn poll(&mut self, _manager: &ExternalRuntimeManager) {
        let Ok(event) = self.client.tick().await else {
            return;
        };

        if let NetworkClientEvent::Payload(payload) = event {
            let _ = self.router.dispatch_bytes(payload.as_bytes()).await;
        }
    }
}
