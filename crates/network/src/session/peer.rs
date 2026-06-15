use std::net::SocketAddr;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct NetworkPeerId(SocketAddr);

impl NetworkPeerId {
    pub const fn new(addr: SocketAddr) -> Self {
        Self(addr)
    }

    pub const fn addr(self) -> SocketAddr {
        self.0
    }
}

impl From<SocketAddr> for NetworkPeerId {
    fn from(addr: SocketAddr) -> Self {
        Self::new(addr)
    }
}
