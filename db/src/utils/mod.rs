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

use crate::utils::enums::{Engine, KeyType, Tag};

pub mod comm;
mod comm_test;
pub mod deploy;
mod deploy_test;
pub mod enums;
pub mod path;
mod path_test;
pub mod store;
mod store_test;
pub mod writer;

pub struct Enum {}

pub trait EnumHandler {
    fn tag_u8(tag: Tag) -> u8;
    fn engine_u8(index_type: Engine) -> u8;
    fn key_type_u8(key_type: KeyType) -> u8;
    fn tag(b: u8) -> Tag;
    fn engine(b: u8) -> Engine;
    fn key_type(b: u8) -> KeyType;
}

pub struct Paths;
