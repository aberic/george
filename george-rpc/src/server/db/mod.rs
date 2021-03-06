/*
 * Copyright (c) 2021. Aberic - All Rights Reserved.
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

use george_db::Task;
use std::sync::Arc;

pub const DATABASE_SYS: &str = "sys";
pub const VIEW_USER: &str = "user";
pub const DEFAULT_COMMENT: &str = "system default";

pub mod database;
pub mod disk;
pub mod index;
pub mod memory;
pub mod page;
pub mod user;
pub mod view;

#[derive(Debug, Clone)]
pub struct DatabaseServer {
    pub task: Arc<Task>,
}

#[derive(Debug, Clone)]
pub struct DiskServer {
    pub task: Arc<Task>,
}

#[derive(Debug, Clone)]
pub struct IndexServer {
    pub task: Arc<Task>,
}

#[derive(Debug, Clone)]
pub struct MemoryServer {
    pub task: Arc<Task>,
}

#[derive(Debug, Clone)]
pub struct PageServer {
    pub task: Arc<Task>,
}

#[derive(Debug, Clone)]
pub struct UserServer {
    pub task: Arc<Task>,
}

#[derive(Debug, Clone)]
pub struct ViewServer {
    pub task: Arc<Task>,
}
