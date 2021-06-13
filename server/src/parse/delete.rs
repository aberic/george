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

use crate::parse::Delete;
use comm::errors::GeorgeResult;
use db::Task;
use std::str::Split;
use std::sync::Arc;

impl Delete {
    pub fn analysis(_task: Arc<Task>, _vss: Vec<String>) -> GeorgeResult<Vec<u8>> {
        unimplemented!()
    }
}
