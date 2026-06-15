#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NetworkPayload {
    bytes: Vec<u8>,
}

impl NetworkPayload {
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            bytes: bytes.into(),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl From<Vec<u8>> for NetworkPayload {
    fn from(bytes: Vec<u8>) -> Self {
        Self::new(bytes)
    }
}

impl From<&[u8]> for NetworkPayload {
    fn from(bytes: &[u8]) -> Self {
        Self::new(bytes)
    }
}
