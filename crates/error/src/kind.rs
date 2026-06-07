#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ErrorKind {
    Asset,
    Config,
    Gameplay,
    Rendering,
    State,
    Unknown,
}
