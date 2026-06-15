#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct NetworkSessionId(pub u64);

impl NetworkSessionId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}
