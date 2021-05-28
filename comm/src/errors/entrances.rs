/*
 * Copyright (c) 2020. Aberic - All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::error::Error;
use std::fmt::{Display, Formatter, Result};

use crate::errors::children::{
    DataExistError, DataNoExistError, DatabaseExistError, DatabaseNoExistError, DirExistError,
    FileExistError, IndexExistError, IndexNoExistError, MethodNoSupportError, NoneError,
    PageExistError, PageNoExistError, StringError, ViewExistError, ViewNoExistError,
};
use crate::errors::{Errs, GeorgeError, GeorgeString, GeorgeStringErr};

impl Error for GeorgeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            GeorgeError::StringError(ref e) => Some(e),
            GeorgeError::DirExistError(ref e) => Some(e),
            GeorgeError::FileExistError(ref e) => Some(e),
            GeorgeError::DataExistError(ref e) => Some(e),
            GeorgeError::PageExistError(ref e) => Some(e),
            GeorgeError::PageNoExistError(ref e) => Some(e),
            GeorgeError::DatabaseExistError(ref e) => Some(e),
            GeorgeError::ViewExistError(ref e) => Some(e),
            GeorgeError::IndexExistError(ref e) => Some(e),
            GeorgeError::DataNoExistError(ref e) => Some(e),
            GeorgeError::DatabaseNoExistError(ref e) => Some(e),
            GeorgeError::ViewNoExistError(ref e) => Some(e),
            GeorgeError::IndexNoExistError(ref e) => Some(e),
            GeorgeError::MethodNoSupportError(ref e) => Some(e),
            GeorgeError::NoneError(ref e) => Some(e),
        }
    }
}

impl Display for GeorgeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            GeorgeError::StringError(ref e) => e.fmt(f),
            GeorgeError::DirExistError(ref e) => e.fmt(f),
            GeorgeError::FileExistError(ref e) => e.fmt(f),
            GeorgeError::DataExistError(ref e) => e.fmt(f),
            GeorgeError::PageExistError(ref e) => e.fmt(f),
            GeorgeError::PageNoExistError(ref e) => e.fmt(f),
            GeorgeError::DatabaseExistError(ref e) => e.fmt(f),
            GeorgeError::ViewExistError(ref e) => e.fmt(f),
            GeorgeError::IndexExistError(ref e) => e.fmt(f),
            GeorgeError::DataNoExistError(ref e) => e.fmt(f),
            GeorgeError::DatabaseNoExistError(ref e) => e.fmt(f),
            GeorgeError::ViewNoExistError(ref e) => e.fmt(f),
            GeorgeError::IndexNoExistError(ref e) => e.fmt(f),
            GeorgeError::MethodNoSupportError(ref e) => e.fmt(f),
            GeorgeError::NoneError(ref e) => e.fmt(f),
        }
    }
}

impl From<StringError> for GeorgeError {
    fn from(s: StringError) -> Self {
        GeorgeError::StringError(s)
    }
}

impl From<DirExistError> for GeorgeError {
    fn from(s: DirExistError) -> Self {
        GeorgeError::DirExistError(s)
    }
}

impl From<FileExistError> for GeorgeError {
    fn from(s: FileExistError) -> Self {
        GeorgeError::FileExistError(s)
    }
}

impl From<DataExistError> for GeorgeError {
    fn from(s: DataExistError) -> Self {
        GeorgeError::DataExistError(s)
    }
}

impl From<PageExistError> for GeorgeError {
    fn from(s: PageExistError) -> Self {
        GeorgeError::PageExistError(s)
    }
}

impl From<PageNoExistError> for GeorgeError {
    fn from(s: PageNoExistError) -> Self {
        GeorgeError::PageNoExistError(s)
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

impl From<MethodNoSupportError> for GeorgeError {
    fn from(s: MethodNoSupportError) -> Self {
        GeorgeError::MethodNoSupportError(s)
    }
}

impl<T: ToString> GeorgeStringErr<String, T> for GeorgeError {
    fn string(msg: String, err: T) -> Self {
        err_strings(msg, err.to_string())
    }
}

impl<T: ToString> GeorgeStringErr<&str, T> for GeorgeError {
    fn string(msg: &str, err: T) -> Self {
        err_strs(msg, err.to_string())
    }
}

impl GeorgeString<String> for GeorgeError {
    fn string(msg: String) -> Self {
        err_string(msg)
    }
}

impl GeorgeString<&str> for GeorgeError {
    fn string(msg: &str) -> Self {
        err_str(msg)
    }
}

impl Errs {
    pub fn string(msg: String) -> GeorgeError {
        err_string(msg)
    }

    pub fn str(msg: &str) -> GeorgeError {
        err_str(msg)
    }

    pub fn strs<Err: ToString>(msg: &str, err: Err) -> GeorgeError {
        err_strs(msg, err)
    }

    pub fn strings<Err: ToString>(msg: String, err: Err) -> GeorgeError {
        err_strings(msg, err)
    }
}

fn err_string(msg: String) -> GeorgeError {
    GeorgeError::StringError(StringError { error_msg: msg })
}

fn err_str(msg: &str) -> GeorgeError {
    GeorgeError::StringError(StringError {
        error_msg: msg.to_string(),
    })
}

fn err_strs<Err: ToString>(msg: &str, err: Err) -> GeorgeError {
    GeorgeError::StringError(StringError {
        error_msg: format!("{} error: {}", msg, err.to_string()),
    })
}

fn err_strings<Err: ToString>(msg: String, err: Err) -> GeorgeError {
    GeorgeError::StringError(StringError {
        error_msg: format!("{} error: {}", msg, err.to_string()),
    })
}
