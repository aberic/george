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
use crate::utils::Paths;

impl Paths {
    /// 数据根目录 /var/lib/georgedb/data
    pub fn data_path() -> String {
        data_path()
    }

    /// 引导文件目录 /var/lib/georgedb/data/bootstrap.ge
    pub fn bootstrap_filepath() -> String {
        bootstrap_filepath()
    }

    /// 缓存页根目录 /var/lib/georgedb/data/page
    pub fn data_page_path() -> String {
        data_page_path()
    }

    /// 缓存页根目录 /var/lib/georgedb/data/page/page_name
    pub fn page_path(page_name: String) -> String {
        page_path(page_name)
    }

    /// 缓存页根目录 /var/lib/georgedb/data/page/page_name/page.ge
    pub fn page_filepath(page_name: String) -> String {
        page_filepath(page_name)
    }

    /// 库根目录 /var/lib/georgedb/data/database
    pub fn data_database_path() -> String {
        data_database_path()
    }

    /// 库根目录 /var/lib/georgedb/data/database/database_name
    pub fn database_path(database_name: String) -> String {
        database_path(database_name)
    }

    /// 库根目录 /var/lib/georgedb/data/database/database_name/database.ge
    pub fn database_filepath(database_name: String) -> String {
        database_filepath(database_name)
    }

    /// 视图根目录 /var/lib/georgedb/data/database/database_name/view_name
    pub fn view_path(database_name: String, view_name: String) -> String {
        view_path(database_name, view_name)
    }

    /// 视图根目录 /var/lib/georgedb/data/database/database_name/view_name/view.ge
    pub fn view_filepath(database_name: String, view_name: String) -> String {
        view_filepath(database_name, view_name)
    }

    /// 视图根目录 /var/lib/georgedb/data/database/database_name/view_name/index_name
    pub fn index_path(database_name: String, view_name: String, index_name: String) -> String {
        index_path(database_name, view_name, index_name)
    }

    /// 索引文件目录 /var/lib/georgedb/data/database/database_name/view_name/index_name/index.ge
    pub fn index_filepath(database_name: String, view_name: String, index_name: String) -> String {
        index_filepath(database_name, view_name, index_name)
    }

    /// 索引文件目录 /var/lib/georgedb/data/database/database_name/view_name/index_name/index_file_name.ge
    pub fn node_filepath(index_path: String, index_file_name: String) -> String {
        node_filepath(index_path, index_file_name)
    }

    /// 索引文件目录 /var/lib/georgedb/data/database/database_name/view_name/index_name/record.ge
    pub fn record_filepath(index_path: String) -> String {
        record_filepath(index_path)
    }
}

/// 数据根目录 /var/lib/georgedb/data
fn data_path() -> String {
    format!("{}/data", GLOBAL_CONFIG.read().unwrap().data_dir.clone(),)
}

/// 缓存页根目录 /var/lib/georgedb/data/page
fn data_page_path() -> String {
    format!(
        "{}/data/page",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
    )
}

/// 库根目录 /var/lib/georgedb/data/database
fn data_database_path() -> String {
    format!(
        "{}/data/database",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
    )
}

/// 缓存页根目录 /var/lib/georgedb/data/page/page_name
fn page_path(page_name: String) -> String {
    format!(
        "{}/data/page/{}",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        page_name
    )
}

/// 库根目录 /var/lib/georgedb/data/database/database_name
fn database_path(database_name: String) -> String {
    format!(
        "{}/data/database/{}",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        database_name
    )
}

/// 视图根目录 /var/lib/georgedb/data/database/database_name/view_name
fn view_path(database_name: String, view_name: String) -> String {
    format!(
        "{}/data/database/{}/{}",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        database_name,
        view_name
    )
}

/// 视图根目录 /var/lib/georgedb/data/database/database_name/view_name/index_name
fn index_path(database_name: String, view_name: String, index_name: String) -> String {
    format!(
        "{}/data/database/{}/{}/{}",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        database_name,
        view_name,
        index_name
    )
}

/// 引导文件目录 /var/lib/georgedb/data/bootstrap.ge
fn bootstrap_filepath() -> String {
    format!(
        "{}/{}",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        "bootstrap.ge"
    )
}

/// 缓存页根目录 /var/lib/georgedb/data/page/page_name/page.ge
fn page_filepath(page_name: String) -> String {
    format!(
        "{}/data/page/{}/page.ge",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        page_name
    )
}

/// 库根目录 /var/lib/georgedb/data/database/database_name/database.ge
fn database_filepath(database_name: String) -> String {
    format!(
        "{}/data/database/{}/database.ge",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        database_name
    )
}

/// 视图根目录 /var/lib/georgedb/data/database/database_name/view_name/view.ge
fn view_filepath(database_name: String, view_name: String) -> String {
    format!(
        "{}/data/database/{}/{}/view.ge",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        database_name,
        view_name
    )
}

/// 索引文件目录 /var/lib/georgedb/data/database/database_name/view_name/index_name/index.ge
fn index_filepath(database_name: String, view_name: String, index_name: String) -> String {
    format!(
        "{}/index.ge",
        index_path(database_name, view_name, index_name)
    )
}

/// 索引文件目录 /var/lib/georgedb/data/database/database_name/view_name/index_name/index_file_name.ge
fn node_filepath(index_path: String, index_file_name: String) -> String {
    format!("{}/{}.ge", index_path, index_file_name)
}

/// 索引文件目录 /var/lib/georgedb/data/database/database_name/view_name/index_name/record.ge
fn record_filepath(index_path: String) -> String {
    node_filepath(index_path, String::from("record"))
}
