use std::fmt;

#[derive(Debug)]
pub enum NetworkRouteError {
    Decode(cmdproto::Error),
    UnknownCmd(u32),
}

pub type NetworkRouteResult<T> = Result<T, NetworkRouteError>;

impl fmt::Display for NetworkRouteError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decode(error) => write!(formatter, "failed to decode cmdproto packet: {error}"),
            Self::UnknownCmd(cmd) => write!(formatter, "unknown network cmd: {cmd}"),
        }
    }
}

impl std::error::Error for NetworkRouteError {}

impl From<cmdproto::Error> for NetworkRouteError {
    fn from(error: cmdproto::Error) -> Self {
        Self::Decode(error)
    }
}
