#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Recoverable,
    Fatal,
}
