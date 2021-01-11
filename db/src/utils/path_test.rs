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

use crate::utils::path::{
    bootstrap_file_path, data_path, database_path, index_file_path, view_path,
};

#[test]
fn path_test() {
    println!("data_path = {}", data_path());
    println!(
        "database_path = {}",
        database_path(String::from("database"))
    );
    println!(
        "view_path = {}",
        view_path(String::from("database"), String::from("view"))
    );
    println!("bootstrap_file_path = {}", bootstrap_file_path());
    println!(
        "index_file_path = {}",
        index_file_path(
            String::from("database"),
            String::from("view"),
            String::from("index")
        )
    );
}
