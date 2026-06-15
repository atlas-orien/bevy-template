#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NetworkConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
}
