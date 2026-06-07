use std::io;

use crate::{ErrorKind, ErrorSeverity};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("[{kind:?}/{code}] {message}{context}")]
    Message {
        kind: ErrorKind,
        severity: ErrorSeverity,
        code: &'static str,
        message: String,
        context: ErrorContext,
    },
    #[error("[Io/io] {0}")]
    Io(#[from] io::Error),
}

#[derive(Debug, Clone, Default)]
pub struct ErrorContext(pub Option<String>);

impl core::fmt::Display for ErrorContext {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.0 {
            Some(context) => write!(formatter, " ({context})"),
            None => Ok(()),
        }
    }
}

impl GameError {
    pub fn new(severity: ErrorSeverity, code: &'static str, message: impl Into<String>) -> Self {
        Self::Message {
            kind: ErrorKind::Unknown,
            severity,
            code,
            message: message.into(),
            context: ErrorContext::default(),
        }
    }

    pub fn from_kind(kind: ErrorKind, code: &'static str, message: impl Into<String>) -> Self {
        Self::Message {
            kind,
            severity: ErrorSeverity::Recoverable,
            code,
            message: message.into(),
            context: ErrorContext::default(),
        }
    }

    pub fn fatal(kind: ErrorKind, code: &'static str, message: impl Into<String>) -> Self {
        Self::Message {
            kind,
            severity: ErrorSeverity::Fatal,
            code,
            message: message.into(),
            context: ErrorContext::default(),
        }
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        if let Self::Message {
            context: error_context,
            ..
        } = &mut self
        {
            error_context.0 = Some(context.into());
        }

        self
    }

    pub fn kind(&self) -> ErrorKind {
        match self {
            Self::Message { kind, .. } => *kind,
            Self::Io(_) => ErrorKind::Unknown,
        }
    }

    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Self::Message { severity, .. } => *severity,
            Self::Io(_) => ErrorSeverity::Recoverable,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::Message { code, .. } => code,
            Self::Io(_) => "io",
        }
    }
}
