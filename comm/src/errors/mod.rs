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

use crate::errors::children::{
    DataExistError, DataNoExistError, DatabaseExistError, DatabaseNoExistError, DirExistError,
    DirNoExistError, FileExistError, FileNoExistError, IndexExistError, IndexNoExistError,
    MethodNoSupportError, NoneError, PageExistError, PageNoExistError, StringError, ViewExistError,
    ViewNoExistError,
};

pub mod children;
pub mod entrances;
mod entrances_test;

pub trait GeorgeStringErr<M, N>: Sized {
    fn string(_: M, _: N) -> Self;
}

pub trait GeorgeString<M>: Sized {
    fn string(_: M) -> Self;
}

/// 自定义Result类型：GeorgeResult
pub type GeorgeResult<T> = std::result::Result<T, GeorgeError>;

/// 索引触发Error,实现std::fmt::Debug的trait
#[derive(Debug)]
pub enum GeorgeError {
    StringError(StringError),
    DirExistError(DirExistError),
    FileExistError(FileExistError),
    DataExistError(DataExistError),
    PageExistError(PageExistError),
    PageNoExistError(PageNoExistError),
    DatabaseExistError(DatabaseExistError),
    ViewExistError(ViewExistError),
    IndexExistError(IndexExistError),
    DirNoExistError(DirNoExistError),
    FileNoExistError(FileNoExistError),
    DataNoExistError(DataNoExistError),
    DatabaseNoExistError(DatabaseNoExistError),
    ViewNoExistError(ViewNoExistError),
    IndexNoExistError(IndexNoExistError),
    MethodNoSupportError(MethodNoSupportError),
    NoneError(NoneError),
}

pub struct Errs;
