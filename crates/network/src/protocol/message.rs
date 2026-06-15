use crate::session::{NetworkPeerId, NetworkSessionId};

use super::NetworkPayload;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NetworkInboundMessage {
    pub peer: NetworkPeerId,
    pub session: Option<NetworkSessionId>,
    pub payload: NetworkPayload,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NetworkOutboundMessage {
    pub peer: NetworkPeerId,
    pub session: Option<NetworkSessionId>,
    pub payload: NetworkPayload,
}

impl NetworkInboundMessage {
    pub fn new(
        peer: NetworkPeerId,
        session: Option<NetworkSessionId>,
        payload: impl Into<NetworkPayload>,
    ) -> Self {
        Self {
            peer,
            session,
            payload: payload.into(),
        }
    }
}

impl NetworkOutboundMessage {
    pub fn new(
        peer: NetworkPeerId,
        session: Option<NetworkSessionId>,
        payload: impl Into<NetworkPayload>,
    ) -> Self {
        Self {
            peer,
            session,
            payload: payload.into(),
        }
    }
}
