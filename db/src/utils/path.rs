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

use crate::utils::deploy::GLOBAL_CONFIG;

/// 数据根目录 /var/lib/georgedb/data
pub fn data_path() -> String {
    format!(
        "{}/{}",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        "data"
    )
}

/// 库根目录 /var/lib/georgedb/data/database
pub fn database_path(database_id: String) -> String {
    format!(
        "{}/{}/{}",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        "data",
        database_id
    )
}

/// 视图根目录 /var/lib/georgedb/data/database/view
pub fn view_path(database_id: String, view_id: String) -> String {
    format!(
        "{}/{}/{}/{}",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        "data",
        database_id,
        view_id
    )
}

/// 引导文件目录 /var/lib/georgedb/data/bootstrap.sr
pub fn bootstrap_file_path() -> String {
    format!(
        "{}/{}",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        "bootstrap.sr"
    )
}

/// 库根目录 /var/lib/georgedb/data/database/db.sr
pub fn database_file_path(database_id: String) -> String {
    format!(
        "{}/{}/{}/db.sr",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        "data",
        database_id
    )
}

/// 视图根目录 /var/lib/georgedb/data/database/view/view.sr
pub fn view_file_path(database_id: String, view_id: String) -> String {
    format!(
        "{}/{}/{}/{}/view.sr",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        "data",
        database_id,
        view_id
    )
}

/// 索引文件目录 /var/lib/georgedb/data/database/view/index.sr
pub fn index_file_path(database_id: String, view_id: String, index_name: String) -> String {
    format!(
        "{}/{}/{}/{}/{}.sr",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        "data",
        database_id,
        view_id,
        index_name
    )
}

/// 索引文件目录 /var/lib/georgedb/data/database/view/index.sr
pub fn index_file_path_yet(
    database_id: String,
    view_id: String,
    index_file_name: String,
) -> String {
    format!(
        "{}/{}/{}/{}/{}",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        "data",
        database_id,
        view_id,
        index_file_name
    )
}
