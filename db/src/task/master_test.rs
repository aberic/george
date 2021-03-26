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

use chrono::{Duration, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::task::master::GLOBAL_MASTER;
use crate::utils::comm::INDEX_SEQUENCE;
use crate::utils::enums::{IndexType, KeyType};
use comm::strings::{StringHandler, Strings};
use std::error::Error;

#[test]
fn database_map_test() {
    database_map();
}

#[test]
fn base_test() {
    // database_create_test
    let database_name = "database_create_base_test1";
    create_database(database_name);
    // database_modify_test
    let database_name = "database_modify_base_test2";
    let database_new_name = "database_modify_base_test3";
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
    create_view(database_name, view_name);
    modify_view(database_name, view_name, view_new_name);
    modify_view(database_name, view_name, view_new_name);
    // index_create_test
    let database_name = "database_index_create_test";
    let view_name = "view_index_create_test";
    let index_name = "index_create_test";
    create_index(
        database_name,
        view_name,
        index_name,
        IndexType::Library,
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
    create_view("database_view_create_test", "view_create_test");
    database_map();
}

#[test]
fn view_modify_test() {
    create_view("database_view_modify_test", "view_modify_test1");
    modify_view(
        "database_view_modify_test",
        "view_modify_test1",
        "view_modify_test2",
    );
    database_map();
}

#[test]
fn view_archive_test() {
    create_view("database_view_archive_test", "view_archive_test1");
    archive_view(
        "database_view_archive_test",
        "view_archive_test1",
        "src/test/dir/x.ge",
    );
    database_map();
}

#[test]
fn index_create_test() {
    create_index(
        "database_index_create_test",
        "view_index_create_test",
        "index_create_test",
        IndexType::Library,
        KeyType::String,
        false,
        true,
        false,
    );
    database_map();
}

#[test]
fn memory_test() {
    let key1 = "a";
    let key2 = "b";
    let key3 = "c";
    put_m(key1, "test1", 1);
    put_m(key2, "test2", 2);
    get_m(key1, 1);
    get_m(key2, 2);
    remove_m(key2, 3);
    get_m(key2, 3);
    put_m(key3, "test4", 4);
    get_m(key3, 4);
    put_m(key3, "test5", 5);
    get_m(key3, 5);
    set_m(key3, "test6", 6);
    get_m(key3, 6);
}

#[test]
fn index_test_prepare() {
    let database_name = "database_index_test";
    let view_name = "view_index_test";
    create_view(database_name, view_name);
    let mut i = 1;
    while i < 5 {
        // 循环体
        put(database_name, view_name, i.to_string().as_str(), "world", i);
        get(database_name, view_name, i.to_string().as_str(), i);
        i += 1;
    }
}

#[test]
fn index_test() {
    let database_name = "database_index_test";
    let view_name = "view_index_test";
    let mut i = 1;
    while i < 5 {
        // 循环体
        get(database_name, view_name, i.to_string().as_str(), i);
        i += 1;
    }
}

#[test]
fn sequence_test() {
    let database_name = "database_sequence_base_test";
    let view_name = "view_sequence_base_test";
    create_view(database_name, view_name);
    let mut i = 1;
    while i < 5 {
        // 循环体
        put(database_name, view_name, "", "world", i);
        get_by_index(
            database_name,
            view_name,
            INDEX_SEQUENCE,
            i.to_string().as_str(),
            i,
        );
        i += 1;
    }
}

#[test]
fn sequence_test_after() {
    let database_name = "database_sequence_base_test";
    let view_name = "view_sequence_base_test";
    put(
        database_name,
        view_name,
        "7",
        "hello12345hello67890world12345world67890",
        1,
    );
    get_by_index(database_name, view_name, INDEX_SEQUENCE, "1", 1);
    get_by_index(database_name, view_name, INDEX_SEQUENCE, "2", 2);
    get_by_index(database_name, view_name, INDEX_SEQUENCE, "3", 3);
    get_by_index(database_name, view_name, INDEX_SEQUENCE, "4", 4);
    get_by_index(database_name, view_name, INDEX_SEQUENCE, "5", 5);
    get_by_index(database_name, view_name, INDEX_SEQUENCE, "6", 6);
    get_by_index(database_name, view_name, INDEX_SEQUENCE, "7", 7);
    get_by_index(database_name, view_name, INDEX_SEQUENCE, "8", 8);
}

#[test]
fn sequence_test_delete() {
    let database_name = "database_sequence_base_test";
    let view_name = "view_sequence_base_test";
    del(database_name, view_name, "2", 2);
    get_by_index(database_name, view_name, INDEX_SEQUENCE, "1", 1);
    get_by_index(database_name, view_name, INDEX_SEQUENCE, "2", 2);
    get_by_index(database_name, view_name, INDEX_SEQUENCE, "3", 3);
    get_by_index(database_name, view_name, INDEX_SEQUENCE, "4", 4);
    get_by_index(database_name, view_name, INDEX_SEQUENCE, "5", 5);
    get_by_index(database_name, view_name, INDEX_SEQUENCE, "6", 6);
    get_by_index(database_name, view_name, INDEX_SEQUENCE, "7", 7);
    get_by_index(database_name, view_name, INDEX_SEQUENCE, "8", 8);
    get_by_index(database_name, view_name, INDEX_SEQUENCE, "9", 9);
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

#[test]
fn select_document_sequence_prepare() {
    let database_name = "database_select_sequence_base_test";
    let view_name = "view_select_base_test";
    create_view(database_name, view_name);

    let mut pos1: u32 = 1;
    while pos1 <= 100000 {
        print!("{} ", pos1);
        let user_str = serde_json::to_string(&create_t(pos1, 100000 - pos1)).unwrap();
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
fn select_document_select_sequence1() {
    let database_name = "database_select_sequence_base_test";
    let view_name = "view_select_base_test";
    let cond_str0 = r#"
  {
    "Conditions":[
        {
            "Param":"george_db_index_sequence",
            "Cond":"ge",
            "Value":49900
        },
        {
            "Param":"age",
            "Cond":"ge",
            "Value":49900,
            "Type": "i64"
        },
        {
            "Param":"age",
            "Cond":"le",
            "Value":90100
        }
    ],
    "Sort":{
        "Param":"height",
        "Asc":true
    },
    "Skip":100,
    "Limit":1000
  }"#;
    select(database_name, view_name, cond_str0.as_bytes().to_vec(), 0);
}

#[test]
fn select_document_select_sequence2() {
    let database_name = "database_select_sequence_base_test";
    let view_name = "view_select_base_test";
    let cond_str0 = r#"
  {
    "Conditions":[
        {
            "Param":"age",
            "Cond":"ge",
            "Value":49900,
            "Type": "i64"
        },
        {
            "Param":"age",
            "Cond":"le",
            "Value":50100
        }
    ],
    "Sort":{
        "Param":"height",
        "Asc":false
    },
    "Skip":100,
    "Limit":1000
  }"#;
    select(database_name, view_name, cond_str0.as_bytes().to_vec(), 0);
}

#[test]
fn select_document_delete_sequence1() {
    let database_name = "database_select_sequence_base_test";
    let view_name = "view_select_base_test";
    let cond_str0 = r#"
  {
    "Conditions":[
        {
            "Param":"george_db_index_sequence",
            "Cond":"ge",
            "Value":49900
        },
        {
            "Param":"age",
            "Cond":"ge",
            "Value":49900,
            "Type": "i64"
        },
        {
            "Param":"age",
            "Cond":"le",
            "Value":90100
        }
    ],
    "Sort":{
        "Param":"height",
        "Asc":true
    },
    "Skip":100,
    "Limit":1000
  }"#;
    delete(database_name, view_name, cond_str0.as_bytes().to_vec(), 0);
}

#[test]
fn index_catalog_sequence_test_prepare() {
    let database_name = "database_index_catalog_sequence_test";
    let view_name = "view_index_test";
    create_view(database_name, view_name);

    let mut pos1: u32 = 1;
    while pos1 <= 100000 {
        print!("{} ", pos1);
        let user_str = serde_json::to_string(&create_t(pos1, 100000 - pos1)).unwrap();
        put(
            database_name,
            view_name,
            format!("key{}", pos1).as_str(),
            user_str.as_str(),
            pos1 as usize,
        );
        pos1 += 1
    }
}

#[test]
fn index_catalog_sequence_test_select1() {
    let database_name = "database_index_catalog_sequence_test";
    let view_name = "view_index_test";
    let cond_str0 = r#"
  {
    "Conditions":[
        {
            "Param":"george_db_index_sequence",
            "Cond":"ge",
            "Value":49900
        },
        {
            "Param":"age",
            "Cond":"ge",
            "Value":49900,
            "Type": "i64"
        },
        {
            "Param":"age",
            "Cond":"le",
            "Value":90100
        }
    ],
    "Sort":{
        "Param":"height",
        "Asc":true
    },
    "Skip":100,
    "Limit":1000
  }"#;
    select(database_name, view_name, cond_str0.as_bytes().to_vec(), 0);
}

#[test]
fn library_index_test() {
    create_index(
        "database_library_index_test",
        "view_index_test",
        "age",
        IndexType::Library,
        KeyType::U32,
        false,
        true,
        false,
    );
    database_map();
}

fn database_map() {
    for (database_name, db) in GLOBAL_MASTER
        .database_map()
        .read()
        .unwrap()
        .iter()
        .into_iter()
    {
        let db_c = db.clone();
        let db_r = db_c.read().unwrap();

        let duration: Duration = db_r.create_time();
        let time_from_stamp = NaiveDateTime::from_timestamp(duration.num_seconds(), 0);

        let time_format = time_from_stamp.format("%Y-%m-%d %H:%M:%S");

        println!(
            "database_map_test {} | {} | {} | {}",
            database_name,
            db_r.name(),
            db_r.create_time(),
            time_format
        );

        for (view_name, view) in db_r.view_map().read().unwrap().iter().into_iter() {
            let view_c = view.clone();
            let view_r = view_c.read().unwrap();

            let duration: Duration = view_r.create_time();
            let time_from_stamp = NaiveDateTime::from_timestamp(duration.num_seconds(), 0);

            let time_format = time_from_stamp.format("%Y-%m-%d %H:%M:%S");

            println!(
                "view_map_test {} | {} | {} | {}",
                view_name,
                view_r.name(),
                view_r.create_time(),
                time_format
            );

            for (index_name, index) in view_r.index_map().read().unwrap().iter().into_iter() {
                let index_c = index.clone();
                let index_r = index_c.read().unwrap();

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

fn create_database(database_name: &str) {
    match GLOBAL_MASTER.create_database(String::from(database_name), String::from("comment")) {
        Ok(()) => println!("create database {}", database_name),
        Err(err) => println!("create database {} error, {}", database_name, err),
    }
}

fn modify_database(database_name: &str, database_new_name: &str) {
    match GLOBAL_MASTER
        .modify_database(String::from(database_name), String::from(database_new_name))
    {
        Ok(()) => println!("modify database {} to {}", database_name, database_new_name),
        Err(err) => println!(
            "modify database {} to {} error, {}",
            database_name, database_new_name, err
        ),
    }
}

fn create_view(database_name: &str, view_name: &str) {
    create_database(database_name.clone());
    match GLOBAL_MASTER.create_view(
        String::from(database_name),
        String::from(view_name),
        String::from("comment"),
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
    create_view(database_name.clone(), view_name.clone());
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
    match GLOBAL_MASTER.put(
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
    match GLOBAL_MASTER.set(
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
    match GLOBAL_MASTER.get(
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
    match GLOBAL_MASTER.remove(
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
    match GLOBAL_MASTER.get_by_index(
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

fn remove(database_name: &str, view_name: &str, key: &str, position: usize) {
    match GLOBAL_MASTER.remove(
        database_name.to_string(),
        view_name.to_string(),
        key.to_string(),
    ) {
        Ok(_) => println!("remove{} success!", position),
        Err(ie) => println!(
            "remove{} is {:#?}",
            position,
            ie.source().unwrap().to_string()
        ),
    }
}

fn put_m(key: &str, value: &str, position: usize) {
    match GLOBAL_MASTER.put_m(key.to_string(), value.to_string().into_bytes()) {
        Err(ie) => println!(
            "put{} error is {:#?}",
            position,
            ie.source().unwrap().to_string()
        ),
        _ => {}
    }
}

fn set_m(key: &str, value: &str, position: usize) {
    match GLOBAL_MASTER.set_m(key.to_string(), value.to_string().into_bytes()) {
        Err(ie) => println!(
            "put{} error is {:#?}",
            position,
            ie.source().unwrap().to_string()
        ),
        _ => {}
    }
}

fn get_m(key: &str, position: usize) {
    match GLOBAL_MASTER.get_m(key.to_string()) {
        Ok(vu8) => println!(
            "get{} is {:#?}",
            position,
            String::from_utf8(vu8).unwrap().as_str()
        ),
        Err(ie) => println!("get{} is {:#?}", position, ie.source().unwrap().to_string()),
    }
}

fn remove_m(key: &str, position: usize) {
    match GLOBAL_MASTER.remove_m(key.to_string()) {
        Ok(_) => println!("remove{} success!", position),
        Err(ie) => println!(
            "remove{} is {:#?}",
            position,
            ie.source().unwrap().to_string()
        ),
    }
}

fn select(database_name: &str, view_name: &str, constraint_json_bytes: Vec<u8>, position: usize) {
    match GLOBAL_MASTER.select(
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
    match GLOBAL_MASTER.delete(
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
