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

use crate::task::engine::memory::index::Index;
use crate::task::engine::memory::seed::Seed;
use std::error::Error;
use std::sync::{Arc, RwLock};

#[test]
fn put_get() {
    let index = Index::create(String::from("index")).unwrap();
    let key = "test".to_string();
    let seed = Arc::new(RwLock::new(Seed::create(
        key.clone(),
        key.clone().into_bytes(),
    )));
    index.write().unwrap().put(key.clone(), seed).unwrap();
    let irg = index.read().unwrap().get(key.clone());
    match irg {
        Ok(seed) => println!("u1 is {:#?}", seed),
        Err(ie) => println!("res1 is {:#?}", ie.source().unwrap().to_string()),
    }
}
