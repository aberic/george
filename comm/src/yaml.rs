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

use crate::errors::entrances::{GeorgeResult, err_str_enhance};
use serde::de::DeserializeOwned;

pub fn string_2_yaml<T: DeserializeOwned>(data: String) -> GeorgeResult<T> {
    let t: T;
    match serde_yaml::from_str(&data) {
        Ok(serde_t) => {
            t = serde_t;
            Ok(t)
        }
        Err(err) => Err(err_str_enhance("serde_yaml_from_str", err.to_string())),
    }
}
