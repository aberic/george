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

use crate::parse::Parse;
use comm::errors::GeorgeResult;
use db::Task;
use std::sync::Arc;

impl Parse {
    pub fn analysis(_task: Arc<Task>, scan: String) -> GeorgeResult<Vec<u8>> {
        Ok(scan.as_bytes().to_vec())
    }
}
