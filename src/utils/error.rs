#[derive(Debug, Clone, PartialEq)]
pub enum SparserError {
    MemoryError(String),
    ParseError(String),
}

pub type Result<T> = ::std::result::Result<T, SparserError>;