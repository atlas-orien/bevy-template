use std::net::SocketAddr;
use std::time::Duration;

use network::connection::{
    NetworkClient, NetworkClientConfig, NetworkClientEvent, NetworkConnectionState,
};
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
    next_seq: u32,
    last_logged_state: NetworkConnectionState,
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

        let last_logged_state = client.state();

        Some(Self {
            client,
            router: demo_toc_router(),
            next_seq: 1,
            last_logged_state,
        })
    }

    pub fn send_demo_login_request(&mut self) {
        let seq = self.allocate_seq();
        match network::request::login(seq, "alice", "secret") {
            Ok(payload) => match self.client.send(payload) {
                Ok(true) => {
                    println!("network login request sent: seq={seq}");
                }
                Ok(false) => {
                    println!("network login request queued=false: seq={seq}");
                }
                Err(error) => {
                    println!("network login request send failed: {error:?}");
                }
            },
            Err(error) => {
                println!("network login request encode failed: {error}");
            }
        }
    }

    pub async fn poll(&mut self, _manager: &ExternalRuntimeManager) {
        let event = match self.client.tick().await {
            Ok(event) => event,
            Err(error) => {
                println!("network client tick failed: {error:?}");
                return;
            }
        };

        match event {
            NetworkClientEvent::Connected => {
                self.log_state_change(
                    NetworkConnectionState::Connected,
                    "network client connected",
                );
            }
            NetworkClientEvent::Payload(payload) => {
                if let Err(error) = self.router.dispatch_bytes(payload.as_bytes()).await {
                    println!("network payload dispatch failed: {error}");
                }
            }
            NetworkClientEvent::SendFailed => {
                println!("network send failed; reconnect scheduled");
            }
            NetworkClientEvent::TransportUnavailable { kind } => {
                println!("network transport unavailable ({kind:?}); reconnect scheduled");
            }
            NetworkClientEvent::Reconnecting => {
                self.log_state_change(
                    NetworkConnectionState::Reconnecting,
                    "network client reconnecting",
                );
            }
            NetworkClientEvent::Idle => {}
        }
    }

    fn allocate_seq(&mut self) -> u32 {
        let seq = self.next_seq;
        self.next_seq = self.next_seq.wrapping_add(1).max(1);
        seq
    }

    fn log_state_change(&mut self, state: NetworkConnectionState, message: &str) {
        if self.last_logged_state != state {
            self.last_logged_state = state;
            println!("{message}");
        }
    }
}
