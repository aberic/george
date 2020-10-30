use std::error::Error;
use std::fmt::{Display, Formatter, Result};

use crate::errors::children::{
    DataExistError, DataNoExistError, DatabaseExistError, DatabaseNoExistError, IndexExistError,
    IndexNoExistError, NoneError, StringError, ViewExistError, ViewNoExistError,
};

/// 自定义Result类型：GeorgeResult
pub type GeorgeResult<T> = std::result::Result<T, GeorgeError>;

/// 索引触发Error,实现std::fmt::Debug的trait
#[derive(Debug)]
pub enum GeorgeError {
    DataExistError(DataExistError),
    DatabaseExistError(DatabaseExistError),
    ViewExistError(ViewExistError),
    IndexExistError(IndexExistError),
    DataNoExistError(DataNoExistError),
    DatabaseNoExistError(DatabaseNoExistError),
    ViewNoExistError(ViewNoExistError),
    IndexNoExistError(IndexNoExistError),
    StringError(StringError),
    NoneError(NoneError),
}

impl Error for GeorgeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            GeorgeError::DataExistError(ref e) => Some(e),
            GeorgeError::DatabaseExistError(ref e) => Some(e),
            GeorgeError::ViewExistError(ref e) => Some(e),
            GeorgeError::IndexExistError(ref e) => Some(e),
            GeorgeError::DataNoExistError(ref e) => Some(e),
            GeorgeError::DatabaseNoExistError(ref e) => Some(e),
            GeorgeError::ViewNoExistError(ref e) => Some(e),
            GeorgeError::IndexNoExistError(ref e) => Some(e),
            GeorgeError::StringError(ref e) => Some(e),
            GeorgeError::NoneError(ref e) => Some(e),
        }
    }
}

impl Display for GeorgeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            GeorgeError::DataExistError(ref e) => e.fmt(f),
            GeorgeError::DatabaseExistError(ref e) => e.fmt(f),
            GeorgeError::ViewExistError(ref e) => e.fmt(f),
            GeorgeError::IndexExistError(ref e) => e.fmt(f),
            GeorgeError::DataNoExistError(ref e) => e.fmt(f),
            GeorgeError::DatabaseNoExistError(ref e) => e.fmt(f),
            GeorgeError::ViewNoExistError(ref e) => e.fmt(f),
            GeorgeError::IndexNoExistError(ref e) => e.fmt(f),
            GeorgeError::StringError(ref e) => e.fmt(f),
            GeorgeError::NoneError(ref e) => e.fmt(f),
        }
    }
}

impl From<DataExistError> for GeorgeError {
    fn from(s: DataExistError) -> Self {
        GeorgeError::DataExistError(s)
    }
}

impl From<DatabaseExistError> for GeorgeError {
    fn from(s: DatabaseExistError) -> Self {
        GeorgeError::DatabaseExistError(s)
    }
}

impl From<ViewExistError> for GeorgeError {
    fn from(s: ViewExistError) -> Self {
        GeorgeError::ViewExistError(s)
    }
}

impl From<IndexExistError> for GeorgeError {
    fn from(s: IndexExistError) -> Self {
        GeorgeError::IndexExistError(s)
    }
}

impl From<DataNoExistError> for GeorgeError {
    fn from(s: DataNoExistError) -> Self {
        GeorgeError::DataNoExistError(s)
    }
}

impl From<DatabaseNoExistError> for GeorgeError {
    fn from(s: DatabaseNoExistError) -> Self {
        GeorgeError::DatabaseNoExistError(s)
    }
}

impl From<ViewNoExistError> for GeorgeError {
    fn from(s: ViewNoExistError) -> Self {
        GeorgeError::ViewNoExistError(s)
    }
}

impl From<IndexNoExistError> for GeorgeError {
    fn from(s: IndexNoExistError) -> Self {
        GeorgeError::IndexNoExistError(s)
    }
}

impl From<NoneError> for GeorgeError {
    fn from(s: NoneError) -> Self {
        GeorgeError::NoneError(s)
    }
}

impl From<StringError> for GeorgeError {
    fn from(s: StringError) -> Self {
        GeorgeError::StringError(s)
    }
}

pub fn err_string(msg: String) -> GeorgeError {
    GeorgeError::StringError(StringError { error_msg: msg })
}

pub fn err_str(msg: &str) -> GeorgeError {
    GeorgeError::StringError(StringError {
        error_msg: msg.to_string(),
    })
}

pub fn err_str_enhance(msg: &str, err: String) -> GeorgeError {
    GeorgeError::StringError(StringError {
        error_msg: format!("{}! error is {}", msg, err),
    })
}
