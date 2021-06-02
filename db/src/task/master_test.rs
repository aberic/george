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

use std::error::Error;

use chrono::{Duration, NaiveDateTime};
use serde::{Deserialize, Serialize};

use comm::strings::StringHandler;
use comm::Strings;

use crate::task::engine::traits::TForm;
use crate::task::GLOBAL_MASTER;
use crate::utils::enums::{IndexType, KeyType};

#[cfg(test)]
mod master {
    #[cfg(test)]
    mod base {
        use crate::task::master_test::{
            archive_view, create_database, create_index, create_page, create_view,
            create_view_with_increment, database_map, modify_database, modify_page, modify_view,
            view_record,
        };
        use crate::utils::enums::{IndexType, KeyType};

        #[test]
        fn test() {
            // database_create_test
            let database_name = "database_create_base_test";
            create_database(database_name);
            // database_modify_test
            let database_name = "database_modify_base_test1";
            let database_new_name = "database_modify_base_test2";
            create_database(database_name);
            modify_database(database_name, database_new_name);
            modify_database(database_name, database_new_name);
            // view_create_test
            let database_name = "database_view_create_base_test";
            let view_name = "view_create_base_test";
            create_view(database_name, view_name);
            // view_modify_test
            let database_name = "database_view_modify_base_test";
            let view_name = "view_modify_base_test1";
            let view_new_name = "view_modify_base_test2";
            create_view_with_increment(database_name, view_name);
            modify_view(database_name, view_name, view_new_name);
            modify_view(database_name, view_name, view_new_name);
            // page_test
            let page_name = "page_modify_base_test1";
            let page_new_name = "page_modify_base_test2";
            create_page(page_name);
            modify_page(page_name, page_new_name);
            // index_create_test
            let database_name = "database_index_create_test";
            let view_name = "view_index_create_test";
            let index_name = "index_create_dossier_test";
            create_index(
                database_name,
                view_name,
                index_name,
                IndexType::Disk,
                KeyType::String,
                false,
                true,
                false,
            );
            database_map();
        }

        #[test]
        fn database_create_test() {
            create_database("database_create_test");
            database_map();
        }

        #[test]
        fn database_modify_test() {
            create_database("database_modify_test1");
            modify_database("database_modify_test1", "database_modify_test2");
            database_map();
        }

        #[test]
        fn view_create_test() {
            create_view_with_increment("database_view_create_test", "view_create_test");
            database_map();
        }

        #[test]
        fn view_modify_test() {
            create_view_with_increment("database_view_modify_test", "view_modify_test1");
            modify_view(
                "database_view_modify_test",
                "view_modify_test1",
                "view_modify_test2",
            );
            database_map();
        }

        #[test]
        fn view_archive_test() {
            create_view_with_increment("database_view_archive_test", "view_archive_test");
            archive_view(
                "database_view_archive_test",
                "view_archive_test",
                "src/test/dir/x.ge",
            );
            database_map();
        }

        #[test]
        fn view_record_test() {
            view_record("database_view_archive_test", "view_archive_test", 0)
        }

        #[test]
        fn database_map_test() {
            database_map();
        }
    }

    #[cfg(test)]
    mod index {
        use crate::task::master_test::{create_view_with_increment, get, put};

        #[test]
        fn index_with_increment() {
            let database_name = "database_index_test";
            let view_name = "view_index_test";
            create_view_with_increment(database_name, view_name);
            let mut i = 1;
            while i < 5 {
                // 循环体
                put(database_name, view_name, i.to_string().as_str(), "world", i);
                i += 1;
            }

            i = 1;
            while i < 5 {
                // 循环体
                get(database_name, view_name, i.to_string().as_str(), i);
                i += 1;
            }
        }
    }

    #[cfg(test)]
    mod memory {
        use crate::task::master_test::{
            create_page, get_memory, get_memory_default, put_memory, put_memory_default,
            remove_memory, remove_memory_default, set_memory, set_memory_default,
        };

        #[test]
        fn memory_test1() {
            let key1 = "a";
            let key2 = "b";
            let key3 = "c";
            put_memory_default(key1, "test1", 1);
            put_memory_default(key2, "test2", 2);
            get_memory_default(key1, 1);
            get_memory_default(key2, 2);
            remove_memory_default(key2, 3);
            get_memory_default(key2, 3);
            put_memory_default(key3, "test4", 4);
            get_memory_default(key3, 4);
            put_memory_default(key3, "test5", 5);
            get_memory_default(key3, 5);
            set_memory_default(key3, "test6", 6);
            get_memory_default(key3, 6);
        }

        #[test]
        fn memory_test2() {
            let page_name = "page_test2";
            create_page(page_name);
            let key1 = "a";
            let key2 = "b";
            let key3 = "c";
            put_memory(page_name, key1, "test1", 1);
            put_memory(page_name, key2, "test2", 2);
            get_memory(page_name, key1, 1);
            get_memory(page_name, key2, 2);
            remove_memory(page_name, key2, 3);
            get_memory(page_name, key2, 3);
            put_memory(page_name, key3, "test4", 4);
            get_memory(page_name, key3, 4);
            put_memory(page_name, key3, "test5", 5);
            get_memory(page_name, key3, 5);
            set_memory(page_name, key3, "test6", 6);
            get_memory(page_name, key3, 6);
        }
    }

    #[cfg(test)]
    mod disk {
        use crate::task::master_test::{create_view_with_increment, get, put, set};

        #[test]
        fn put_set_get_test() {
            let database_name = "database_disk_base_test";
            let view_name = "view_disk_base_test";
            create_view_with_increment(database_name, view_name);
            put(database_name, view_name, "hello1", "world1", 1);
            put(database_name, view_name, "hello2", "world2", 2);
            put(database_name, view_name, "hello3", "world3", 3);
            get(database_name, view_name, "hello1", 1);
            get(database_name, view_name, "hello2", 2);
            get(database_name, view_name, "hello3", 3);
            set(database_name, view_name, "hello1", "world4", 4);
            get(database_name, view_name, "hello1", 1);
            get(database_name, view_name, "hello2", 2);
            get(database_name, view_name, "hello3", 3);
        }

        #[test]
        fn put_get_1000_test() {
            let database_name = "database_disk_base_test";
            let view_name = "view_disk_base_test";
            let mut pos = 1;
            while pos <= 1000 {
                let key = format!("yes{}", pos);
                let value = format!("no{}", pos);
                put(database_name, view_name, key.as_str(), value.as_str(), pos);
                pos += 1;
            }

            pos = 800;
            while pos <= 840 {
                let key = format!("yes{}", pos);
                get(database_name, view_name, key.as_str(), pos);
                pos += 1;
            }
        }
    }

    #[cfg(test)]
    mod get_by_index {
        use crate::task::master_test::{create_view_with_increment, del, get_by_index, put};
        use crate::utils::comm::INDEX_INCREMENT;

        #[test]
        fn increment_test() {
            let database_name = "database_increment_base_test";
            let view_name = "view_increment_base_test";
            create_view_with_increment(database_name, view_name);
            let mut i = 1;
            while i < 5 {
                // 循环体
                put(
                    database_name,
                    view_name,
                    i.to_string().as_str(),
                    format!("world {}", i).as_str(),
                    i,
                );
                i += 1;
            }

            i = 1;
            while i < 5 {
                // 循环体
                get_by_index(
                    database_name,
                    view_name,
                    INDEX_INCREMENT,
                    i.to_string().as_str(),
                    i,
                );
                i += 1;
            }
        }

        #[test]
        fn increment_test_get() {
            let database_name = "database_increment_base_test";
            let view_name = "view_increment_base_test";
            get_by_index(database_name, view_name, INDEX_INCREMENT, "1", 1);
            get_by_index(database_name, view_name, INDEX_INCREMENT, "2", 2);
            get_by_index(database_name, view_name, INDEX_INCREMENT, "3", 3);
            get_by_index(database_name, view_name, INDEX_INCREMENT, "4", 4);
            get_by_index(database_name, view_name, INDEX_INCREMENT, "5", 5);
            get_by_index(database_name, view_name, INDEX_INCREMENT, "6", 6);
            get_by_index(database_name, view_name, INDEX_INCREMENT, "7", 7);
            get_by_index(database_name, view_name, INDEX_INCREMENT, "8", 8);
        }

        #[test]
        fn increment_test_after() {
            let database_name = "database_increment_base_test";
            let view_name = "view_increment_base_test";
            put(
                database_name,
                view_name,
                "7",
                "hello12345hello67890world12345world67890",
                1,
            );
            get_by_index(database_name, view_name, INDEX_INCREMENT, "1", 1);
            get_by_index(database_name, view_name, INDEX_INCREMENT, "2", 2);
            get_by_index(database_name, view_name, INDEX_INCREMENT, "3", 3);
            get_by_index(database_name, view_name, INDEX_INCREMENT, "4", 4);
            get_by_index(database_name, view_name, INDEX_INCREMENT, "5", 5);
            get_by_index(database_name, view_name, INDEX_INCREMENT, "6", 6);
            get_by_index(database_name, view_name, INDEX_INCREMENT, "7", 7);
            get_by_index(database_name, view_name, INDEX_INCREMENT, "8", 8);
        }

        #[test]
        fn increment_test_delete() {
            let database_name = "database_increment_base_test";
            let view_name = "view_increment_base_test";
            del(database_name, view_name, "2", 2);
            get_by_index(database_name, view_name, INDEX_INCREMENT, "1", 1);
            get_by_index(database_name, view_name, INDEX_INCREMENT, "2", 2);
            get_by_index(database_name, view_name, INDEX_INCREMENT, "3", 3);
            get_by_index(database_name, view_name, INDEX_INCREMENT, "4", 4);
            get_by_index(database_name, view_name, INDEX_INCREMENT, "5", 5);
            get_by_index(database_name, view_name, INDEX_INCREMENT, "6", 6);
            get_by_index(database_name, view_name, INDEX_INCREMENT, "7", 7);
            get_by_index(database_name, view_name, INDEX_INCREMENT, "8", 8);
            get_by_index(database_name, view_name, INDEX_INCREMENT, "9", 9);
        }
    }

    #[cfg(test)]
    mod select_disk {
        use comm::json::JsonHandler;
        use comm::Json;

        use crate::task::master_test::*;

        #[test]
        fn select_disk_prepare() {
            let database_name = "database_select_base_test";
            let view_name = "view_base_test";
            let index_name = "age";
            create_view_with_increment(database_name, view_name);
            create_index(
                database_name,
                view_name,
                index_name,
                IndexType::Disk,
                KeyType::Int,
                false,
                false,
                false,
            );

            let mut pos1: u32 = 1;
            while pos1 <= 10000 {
                print!("{} ", pos1);
                let user_str = Json::obj_2_string(&create_t(pos1, 10000 - pos1)).unwrap();
                put(
                    database_name,
                    view_name,
                    pos1.to_string().as_str(),
                    user_str.as_str(),
                    pos1 as usize,
                );
                pos1 += 1
            }
        }

        #[test]
        fn select_disk_get_by_index() {
            let database_name = "database_select_base_test";
            let view_name = "view_base_test";
            let index_name = "age";
            get_by_index(database_name, view_name, index_name, "1", 1);
            get_by_index(database_name, view_name, index_name, "10", 10);
            get_by_index(database_name, view_name, index_name, "100", 100);
            get_by_index(database_name, view_name, index_name, "1000", 1000);
            get_by_index(database_name, view_name, index_name, "10000", 10000);
            get_by_index(database_name, view_name, index_name, "100000", 100000);
        }

        #[test]
        fn select_increment_left() {
            let database_name = "database_select_base_test";
            let view_name = "view_base_test";
            let cond_str0 = r#"
                                  {
                                    "Conditions":[
                                        {
                                            "Param":"george_db_index_increment",
                                            "Cond":"ge",
                                            "Value":8990
                                        },
                                        {
                                            "Param":"age",
                                            "Cond":"ge",
                                            "Value":4990
                                        },
                                        {
                                            "Param":"age",
                                            "Cond":"le",
                                            "Value":9010
                                        }
                                    ],
                                    "Sort":{
                                        "Param":"height",
                                        "Asc":true
                                    },
                                    "Skip":0,
                                    "Limit":20
                                  }"#;
            select(database_name, view_name, cond_str0.as_bytes().to_vec(), 0);
        }

        #[test]
        fn select_delete_increment() {
            let database_name = "database_select_base_test";
            let view_name = "view_base_test";
            let cond_str0 = r#"
                                  {
                                    "Conditions":[
                                        {
                                            "Param":"george_db_index_increment",
                                            "Cond":"ge",
                                            "Value":4990
                                        },
                                        {
                                            "Param":"age",
                                            "Cond":"ge",
                                            "Value":4990
                                        },
                                        {
                                            "Param":"age",
                                            "Cond":"le",
                                            "Value":9010
                                        }
                                    ],
                                    "Sort":{
                                        "Param":"height",
                                        "Asc":true
                                    },
                                    "Skip":10,
                                    "Limit":100
                                  }"#;
            delete(database_name, view_name, cond_str0.as_bytes().to_vec(), 0);
        }

        #[test]
        fn select_disk_left() {
            let database_name = "database_select_base_test";
            let view_name = "view_base_test";
            let cond_str0 = r#"
                                  {
                                    "Conditions":[
                                        {
                                            "Param":"age",
                                            "Cond":"ge",
                                            "Value":4990
                                        },
                                        {
                                            "Param":"age",
                                            "Cond":"le",
                                            "Value":9010
                                        },
                                        {
                                            "Param":"height",
                                            "Cond":"le",
                                            "Value":5000
                                        }
                                    ],
                                    "Sort":{
                                        "Param":"height",
                                        "Asc":true
                                    },
                                    "Skip":0,
                                    "Limit":20
                                  }"#;
            select(database_name, view_name, cond_str0.as_bytes().to_vec(), 0);
        }

        #[test]
        fn select_disk_right() {
            let database_name = "database_select_base_test";
            let view_name = "view_base_test";
            let cond_str0 = r#"
                                  {
                                    "Conditions":[
                                        {
                                            "Param":"age",
                                            "Cond":"ge",
                                            "Value":4990
                                        },
                                        {
                                            "Param":"age",
                                            "Cond":"le",
                                            "Value":9010
                                        },
                                        {
                                            "Param":"height",
                                            "Cond":"le",
                                            "Value":5000
                                        }
                                    ],
                                    "Sort":{
                                        "Param":"age",
                                        "Asc":false
                                    },
                                    "Skip":0,
                                    "Limit":20
                                  }"#;
            select(database_name, view_name, cond_str0.as_bytes().to_vec(), 0);
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Teacher {
    name: String,
    age: u32,
    height: u32,
    blog: String,
    married: bool,
}

fn create_t(a: u32, h: u32) -> Teacher {
    Teacher {
        name: a.to_string(),
        age: a,
        height: h,
        blog: a.to_string(),
        married: a % 2 == 0,
    }
}

///////////////////////////////////////////////////////////////////////////////////////

fn database_map() {
    db_create_time();
    for (database_name, db) in GLOBAL_MASTER
        .database_map()
        .read()
        .unwrap()
        .iter()
        .into_iter()
    {
        let db_c = db.clone();
        let db_r = db_c.read().unwrap();

        println!(
            "database_map_test {} | {} | {}",
            database_name,
            db_r.name(),
            db_r.create_time().format("%Y-%m-%d %H:%M:%S"),
        );

        for (view_name, view) in db_r.view_map().read().unwrap().iter().into_iter() {
            let view_c = view.clone();
            let view_r = view_c.read().unwrap();

            println!(
                "view_map_test {} | {} | {}",
                view_name,
                view_r.name(),
                view_r.create_time().format("%Y-%m-%d %H:%M:%S"),
            );

            for (index_name, index) in view_r.index_map().read().unwrap().iter().into_iter() {
                let index_r = index.clone();

                let duration: Duration = index_r.create_time();
                let time_from_stamp = NaiveDateTime::from_timestamp(duration.num_seconds(), 0);

                let time_format = time_from_stamp.format("%Y-%m-%d %H:%M:%S");

                println!(
                    "index_map_test {} | {} | {} | {}",
                    index_name,
                    index_r.name(),
                    index_r.create_time(),
                    time_format
                )
            }
        }
    }
}

fn db_create_time() {
    let duration = GLOBAL_MASTER.create_time();
    let time_format = duration.to_string("%Y-%m-%d %H:%M:%S");
    println!("db_create_time = {}", time_format)
}

fn create_database(database_name: &str) {
    match GLOBAL_MASTER.create_database(String::from(database_name), String::from("comment")) {
        Ok(()) => println!("create database {}", database_name),
        Err(err) => println!("create database {} error, {}", database_name, err),
    }
}

fn modify_database(database_name: &str, database_new_name: &str) {
    match GLOBAL_MASTER.modify_database(
        String::from(database_name),
        String::from(database_new_name),
        String::from("comment"),
    ) {
        Ok(()) => println!("modify database {} to {}", database_name, database_new_name),
        Err(err) => println!(
            "modify database {} to {} error, {}",
            database_name, database_new_name, err
        ),
    }
}

fn create_page(page_name: &str) {
    match GLOBAL_MASTER.create_page(String::from(page_name), String::from("comment")) {
        Ok(()) => println!("create page {}", page_name),
        Err(err) => println!("create page {} error, {}", page_name, err),
    }
}

fn modify_page(page_name: &str, page_new_name: &str) {
    match GLOBAL_MASTER.modify_page(String::from(page_name), String::from(page_new_name)) {
        Ok(()) => println!("modify page {} to {}", page_name, page_new_name),
        Err(err) => println!(
            "modify page {} to {} error, {}",
            page_name, page_new_name, err
        ),
    }
}

fn create_view(database_name: &str, view_name: &str) {
    create_database(database_name.clone());
    match GLOBAL_MASTER.create_view(
        String::from(database_name),
        String::from(view_name),
        String::from("comment"),
        false,
    ) {
        Ok(()) => println!("create view {} from database {}", view_name, database_name),
        Err(err) => println!(
            "create view {} from database {} error, {}",
            view_name, database_name, err
        ),
    }
}

fn create_view_with_increment(database_name: &str, view_name: &str) {
    create_database(database_name.clone());
    match GLOBAL_MASTER.create_view(
        String::from(database_name),
        String::from(view_name),
        String::from("comment"),
        true,
    ) {
        Ok(()) => println!("create view {} from database {}", view_name, database_name),
        Err(err) => println!(
            "create view {} from database {} error, {}",
            view_name, database_name, err
        ),
    }
}

fn modify_view(database_name: &str, view_name: &str, view_new_name: &str) {
    match GLOBAL_MASTER.modify_view(
        String::from(database_name),
        String::from(view_name),
        String::from(view_new_name),
        String::from("comment"),
    ) {
        Ok(()) => println!(
            "modify view {} to {} from {}",
            view_name, view_new_name, database_name
        ),
        Err(err) => println!(
            "modify view {} to {} from {} error, {}",
            view_name, view_new_name, database_name, err
        ),
    }
}

fn archive_view(database_name: &str, view_name: &str, archive_file_path: &str) {
    match GLOBAL_MASTER.archive_view(
        String::from(database_name),
        String::from(view_name),
        String::from(archive_file_path),
    ) {
        Ok(()) => println!("archive view {} success!", view_name),
        Err(err) => println!("archive view {} error: {}", view_name, err),
    }
}

fn view_record(database_name: &str, view_name: &str, version: u16) {
    match GLOBAL_MASTER.view_record(
        String::from(database_name),
        String::from(view_name),
        version,
    ) {
        Ok((filepath, create_time)) => println!(
            "filepath = {}, create_time = {}",
            filepath,
            create_time.format("%Y-%m-%d %H:%M:%S"),
        ),
        Err(err) => println!("archive view {} error: {}", view_name, err),
    }
}

fn create_index(
    database_name: &str,
    view_name: &str,
    index_name: &str,
    index_type: IndexType,
    key_type: KeyType,
    primary: bool,
    unique: bool,
    null: bool,
) {
    create_view_with_increment(database_name.clone(), view_name.clone());
    match GLOBAL_MASTER.create_index(
        String::from(database_name),
        String::from(view_name),
        String::from(index_name),
        index_type,
        key_type,
        primary,
        unique,
        null,
    ) {
        Ok(()) => println!(
            "create index {} from database.view {}.{}",
            index_name, database_name, view_name
        ),
        Err(err) => println!(
            "create index {} from database.view {}.{} error, {}",
            index_name, database_name, view_name, err
        ),
    }
}

fn put(database_name: &str, view_name: &str, key: &str, value: &str, position: usize) {
    match GLOBAL_MASTER.put_disk(
        database_name.to_string(),
        view_name.to_string(),
        key.to_string(),
        value.to_string().into_bytes(),
    ) {
        Err(ie) => println!(
            "put{} error is {:#?}",
            position,
            ie.source().unwrap().to_string()
        ),
        _ => {}
    }
}

fn set(database_name: &str, view_name: &str, key: &str, value: &str, position: usize) {
    match GLOBAL_MASTER.set_disk(
        database_name.to_string(),
        view_name.to_string(),
        key.to_string(),
        value.to_string().into_bytes(),
    ) {
        Err(ie) => println!(
            "put{} error is {:#?}",
            position,
            ie.source().unwrap().to_string()
        ),
        _ => {}
    }
}

fn get(database_name: &str, view_name: &str, key: &str, position: usize) {
    match GLOBAL_MASTER.get_disk(
        database_name.to_string(),
        view_name.to_string(),
        key.to_string(),
    ) {
        Ok(vu8) => println!(
            "get{} is {:#?}",
            position,
            Strings::from_utf8(vu8).unwrap().as_str()
        ),
        Err(ie) => println!("get{} is {:#?}", position, ie.source().unwrap().to_string()),
    }
}

fn del(database_name: &str, view_name: &str, key: &str, position: usize) {
    match GLOBAL_MASTER.remove_disk(
        database_name.to_string(),
        view_name.to_string(),
        key.to_string(),
    ) {
        Ok(_) => println!("del{} success", position,),
        Err(ie) => println!("del{} is {:#?}", position, ie.source().unwrap().to_string()),
    }
}

fn get_by_index(
    database_name: &str,
    view_name: &str,
    index_name: &str,
    key: &str,
    position: usize,
) {
    match GLOBAL_MASTER.get_disk_by_index(
        database_name.to_string(),
        view_name.to_string(),
        index_name.to_string(),
        key.to_string(),
    ) {
        Ok(vu8) => println!(
            "get{} is {:#?}",
            position,
            Strings::from_utf8(vu8).unwrap().as_str()
        ),
        Err(ie) => println!("get{} is {:#?}", position, ie.source().unwrap().to_string()),
    }
}

fn put_memory(page_name: &str, key: &str, value: &str, position: usize) {
    match GLOBAL_MASTER.put_memory(
        page_name.to_string(),
        key.to_string(),
        value.to_string().into_bytes(),
    ) {
        Err(ie) => println!(
            "put{} error is {:#?}",
            position,
            ie.source().unwrap().to_string()
        ),
        _ => {}
    }
}

fn set_memory(page_name: &str, key: &str, value: &str, position: usize) {
    match GLOBAL_MASTER.set_memory(
        page_name.to_string(),
        key.to_string(),
        value.to_string().into_bytes(),
    ) {
        Err(ie) => println!(
            "put{} error is {:#?}",
            position,
            ie.source().unwrap().to_string()
        ),
        _ => {}
    }
}

fn get_memory(page_name: &str, key: &str, position: usize) {
    match GLOBAL_MASTER.get_memory(page_name.to_string(), key.to_string()) {
        Ok(vu8) => println!(
            "get{} is {:#?}",
            position,
            String::from_utf8(vu8).unwrap().as_str()
        ),
        Err(ie) => println!("get{} is {:#?}", position, ie.source().unwrap().to_string()),
    }
}

fn remove_memory(page_name: &str, key: &str, position: usize) {
    match GLOBAL_MASTER.remove_memory(page_name.to_string(), key.to_string()) {
        Ok(_) => println!("remove{} success!", position),
        Err(ie) => println!(
            "remove{} is {:#?}",
            position,
            ie.source().unwrap().to_string()
        ),
    }
}

fn put_memory_default(key: &str, value: &str, position: usize) {
    match GLOBAL_MASTER.put_memory_default(key.to_string(), value.to_string().into_bytes()) {
        Err(ie) => println!(
            "put{} error is {:#?}",
            position,
            ie.source().unwrap().to_string()
        ),
        _ => {}
    }
}

fn set_memory_default(key: &str, value: &str, position: usize) {
    match GLOBAL_MASTER.set_memory_default(key.to_string(), value.to_string().into_bytes()) {
        Err(ie) => println!(
            "put{} error is {:#?}",
            position,
            ie.source().unwrap().to_string()
        ),
        _ => {}
    }
}

fn get_memory_default(key: &str, position: usize) {
    match GLOBAL_MASTER.get_memory_default(key.to_string()) {
        Ok(vu8) => println!(
            "get{} is {:#?}",
            position,
            String::from_utf8(vu8).unwrap().as_str()
        ),
        Err(ie) => println!("get{} is {:#?}", position, ie.source().unwrap().to_string()),
    }
}

fn remove_memory_default(key: &str, position: usize) {
    match GLOBAL_MASTER.remove_memory_default(key.to_string()) {
        Ok(_) => println!("remove{} success!", position),
        Err(ie) => println!(
            "remove{} is {:#?}",
            position,
            ie.source().unwrap().to_string()
        ),
    }
}

fn select(database_name: &str, view_name: &str, constraint_json_bytes: Vec<u8>, position: usize) {
    match GLOBAL_MASTER.select_disk(
        database_name.to_string(),
        view_name.to_string(),
        constraint_json_bytes,
    ) {
        Ok(e) => {
            println!(
                "select{},total={},count={},index_name={},asc={}",
                position, e.total, e.count, e.index_name, e.asc
            );
            for value in e.values {
                println!("value={}", String::from_utf8(value).unwrap());
            }
        }
        Err(ie) => println!(
            "select{} is {:#?}",
            position,
            ie.source().unwrap().to_string()
        ),
    }
}

fn delete(database_name: &str, view_name: &str, constraint_json_bytes: Vec<u8>, position: usize) {
    match GLOBAL_MASTER.delete_disk(
        database_name.to_string(),
        view_name.to_string(),
        constraint_json_bytes,
    ) {
        Ok(e) => {
            println!(
                "delete{},total={},count={},index_name={},asc={}",
                position, e.total, e.count, e.index_name, e.asc
            );
            for value in e.values {
                println!("value={}", String::from_utf8(value).unwrap());
            }
        }
        Err(ie) => println!(
            "delete{} is {:#?}",
            position,
            ie.source().unwrap().to_string()
        ),
    }
}
