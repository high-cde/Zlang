use std::fmt;

pub type ZlangResult<T> = Result<T, ZlangError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZlangError {
    Source {
        line: usize,
        column: usize,
        message: String,
    },
    Compile(String),
    Bytecode(String),
    CapabilityDenied(String),
    ResourceLimit(String),
    Runtime(String),
    Io(String),
    Usage(String),
}

impl ZlangError {
    pub fn source(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::Source {
            line,
            column,
            message: message.into(),
        }
    }

    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Usage(_) => 64,
            Self::Io(_) => 66,
            Self::Source { .. } | Self::Compile(_) => 65,
            Self::Bytecode(_)
            | Self::CapabilityDenied(_)
            | Self::ResourceLimit(_)
            | Self::Runtime(_) => 70,
        }
    }
}

impl fmt::Display for ZlangError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Source {
                line,
                column,
                message,
            } => write!(formatter, "source error at {line}:{column}: {message}"),
            Self::Compile(message) => write!(formatter, "compile error: {message}"),
            Self::Bytecode(message) => write!(formatter, "bytecode error: {message}"),
            Self::CapabilityDenied(message) => write!(formatter, "capability denied: {message}"),
            Self::ResourceLimit(message) => write!(formatter, "resource limit: {message}"),
            Self::Runtime(message) => write!(formatter, "runtime error: {message}"),
            Self::Io(message) => write!(formatter, "I/O error: {message}"),
            Self::Usage(message) => write!(formatter, "usage error: {message}"),
        }
    }
}

impl std::error::Error for ZlangError {}

impl From<std::io::Error> for ZlangError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}
