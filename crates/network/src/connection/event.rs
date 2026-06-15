use std::io::ErrorKind;

use crate::protocol::NetworkPayload;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NetworkClientEvent {
    Connected,
    Payload(NetworkPayload),
    SendFailed,
    TransportUnavailable { kind: ErrorKind },
    Reconnecting,
    Idle,
}
