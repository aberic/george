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

/// 子类型 Error,实现std::fmt::Debug的trait
#[derive(Debug, Clone)]
pub struct StringError {
    pub(crate) error_msg: String,
}

impl Display for StringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.error_msg)
    }
}

impl Error for StringError {}

#[derive(Debug, Clone)]
pub struct DirExistError;

/// 实现Display的trait，并实现fmt方法
impl Display for DirExistError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "dir already exist!")
    }
}

/// 实现Error的trait,因为没有子Error,不需要覆盖source()方法
impl Error for DirExistError {}

#[derive(Debug, Clone)]
pub struct FileExistError;

/// 实现Display的trait，并实现fmt方法
impl Display for FileExistError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "file already exist!")
    }
}

/// 实现Error的trait,因为没有子Error,不需要覆盖source()方法
impl Error for FileExistError {}

#[derive(Debug, Clone)]
pub struct DataExistError;

/// 实现Display的trait，并实现fmt方法
impl Display for DataExistError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "data already exist!")
    }
}

/// 实现Error的trait,因为没有子Error,不需要覆盖source()方法
impl Error for DataExistError {}

#[derive(Debug, Clone)]
pub struct PageExistError;

impl Display for PageExistError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "page already exist!")
    }
}

impl Error for PageExistError {}

#[derive(Debug, Clone)]
pub struct DatabaseExistError;

impl Display for DatabaseExistError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "database already exist!")
    }
}

impl Error for DatabaseExistError {}

#[derive(Debug, Clone)]
pub struct ViewExistError;

impl Display for ViewExistError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "view already exist!")
    }
}

impl Error for ViewExistError {}

#[derive(Debug, Clone)]
pub struct IndexExistError;

impl Display for IndexExistError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "index already exist!")
    }
}

impl Error for IndexExistError {}

#[derive(Debug, Clone)]
pub struct DirNoExistError;

impl Display for DirNoExistError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "dir is not exist!")
    }
}

impl Error for DirNoExistError {}

#[derive(Debug, Clone)]
pub struct FileNoExistError;

impl Display for FileNoExistError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "file is not exist!")
    }
}

impl Error for FileNoExistError {}

#[derive(Debug, Clone)]
pub struct DataNoExistError;

impl Display for DataNoExistError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "data is not exist!")
    }
}

impl Error for DataNoExistError {}

#[derive(Debug, Clone)]
pub struct PageNoExistError;

impl Display for PageNoExistError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "page is not exist!")
    }
}

impl Error for PageNoExistError {}

#[derive(Debug, Clone)]
pub struct DatabaseNoExistError;

impl Display for DatabaseNoExistError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "database is not exist!")
    }
}

impl Error for DatabaseNoExistError {}

#[derive(Debug, Clone)]
pub struct ViewNoExistError;

impl Display for ViewNoExistError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "view is not exist!")
    }
}

impl Error for ViewNoExistError {}

#[derive(Debug, Clone)]
pub struct IndexNoExistError;

impl Display for IndexNoExistError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "index is not exist!")
    }
}

impl Error for IndexNoExistError {}

#[derive(Debug, Clone)]
pub struct MethodNoSupportError;

impl Display for MethodNoSupportError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "method is not support!")
    }
}

impl Error for MethodNoSupportError {}

#[derive(Debug, Clone)]
pub struct NoneError;

impl Display for NoneError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "none err!")
    }
}

impl Error for NoneError {}
