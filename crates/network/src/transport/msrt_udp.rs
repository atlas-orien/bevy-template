use std::io::ErrorKind;

use crate::protocol::NetworkPayload;
use crate::session::NetworkPeerId;
use tokio::net::ToSocketAddrs;

pub type MsrtUdpClient = msrt_udp::UdpClient;
pub type MsrtUdpServer<const N: usize> = msrt_udp::UdpServer<N>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NetworkTransportEvent {
    Payload {
        peer: NetworkPeerId,
        payload: NetworkPayload,
    },
    SendFailed {
        peer: Option<NetworkPeerId>,
    },
    TransportUnavailable {
        kind: ErrorKind,
    },
    Idle,
}

pub async fn bind_client<A, R>(local: A, remote: R) -> msrt_udp::Result<MsrtUdpClient>
where
    A: ToSocketAddrs,
    R: ToSocketAddrs,
{
    msrt_udp::UdpClient::bind(local, remote).await
}

pub async fn bind_server<A, const N: usize>(local: A) -> msrt_udp::Result<MsrtUdpServer<N>>
where
    A: ToSocketAddrs,
{
    msrt_udp::UdpServer::<N>::bind(local).await
}
