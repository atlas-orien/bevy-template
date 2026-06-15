use std::time::{Duration, Instant};

use msrt_udp::{UdpClient, UdpClientEvent};
use tokio::net::ToSocketAddrs;

use crate::connection::{NetworkClientEvent, NetworkConnectionState};
use crate::protocol::NetworkPayload;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct NetworkClientConfig {
    pub reconnect_interval: Duration,
}

impl Default for NetworkClientConfig {
    fn default() -> Self {
        Self {
            reconnect_interval: Duration::from_millis(250),
        }
    }
}

#[derive(Debug)]
pub struct NetworkClient {
    client: UdpClient,
    state: NetworkConnectionState,
    config: NetworkClientConfig,
    next_reconnect: Option<Instant>,
}

impl NetworkClient {
    pub async fn connect<A, R>(local: A, remote: R) -> msrt_udp::Result<Self>
    where
        A: ToSocketAddrs,
        R: ToSocketAddrs,
    {
        Self::connect_with_config(local, remote, NetworkClientConfig::default()).await
    }

    pub async fn connect_with_config<A, R>(
        local: A,
        remote: R,
        config: NetworkClientConfig,
    ) -> msrt_udp::Result<Self>
    where
        A: ToSocketAddrs,
        R: ToSocketAddrs,
    {
        let mut client = UdpClient::bind(local, remote).await?;
        client.connect()?;
        Ok(Self {
            client,
            state: NetworkConnectionState::Connected,
            config,
            next_reconnect: None,
        })
    }

    pub fn state(&self) -> NetworkConnectionState {
        self.state
    }

    pub fn disconnect(&mut self) {
        self.client.disconnect();
        self.state = NetworkConnectionState::Disconnected;
        self.next_reconnect = None;
    }

    pub fn send(&mut self, payload: impl Into<NetworkPayload>) -> msrt_udp::Result<bool> {
        self.client.send(payload.into().as_bytes())
    }

    pub async fn tick(&mut self) -> msrt_udp::Result<NetworkClientEvent> {
        if self.should_reconnect() {
            self.client.connect()?;
            self.state = NetworkConnectionState::Connected;
            self.next_reconnect = None;
            return Ok(NetworkClientEvent::Connected);
        }

        match self.client.tick().await? {
            UdpClientEvent::Message(message) if message.as_bytes() != [0] => {
                self.state = NetworkConnectionState::Connected;
                Ok(NetworkClientEvent::Payload(NetworkPayload::new(
                    message.as_bytes(),
                )))
            }
            UdpClientEvent::Message(_) | UdpClientEvent::Idle => Ok(NetworkClientEvent::Idle),
            UdpClientEvent::SendFailed(_) => {
                self.schedule_reconnect();
                Ok(NetworkClientEvent::SendFailed)
            }
            UdpClientEvent::TransportUnavailable { kind } => {
                self.schedule_reconnect();
                Ok(NetworkClientEvent::TransportUnavailable { kind })
            }
        }
    }

    fn schedule_reconnect(&mut self) {
        self.client.disconnect();
        self.state = NetworkConnectionState::Reconnecting;
        self.next_reconnect = Some(Instant::now() + self.config.reconnect_interval);
    }

    fn should_reconnect(&self) -> bool {
        self.state == NetworkConnectionState::Reconnecting
            && self
                .next_reconnect
                .is_some_and(|deadline| deadline <= Instant::now())
    }
}
