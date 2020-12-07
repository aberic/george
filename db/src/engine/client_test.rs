use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::engine::client::GLOBAL_CLIENT;
use crate::utils::comm::{Category, IndexType, LevelType};

#[test]
fn put_memory() {
    create_database("database", "comment", 1);
    create_database("database", "comment", 2);
    create_database("database1", "comment", 3);
    create_view(
        "database",
        "view",
        "comment",
        IndexType::Siam,
        Category::Memory,
        LevelType::Small,
        1,
    );
    create_view(
        "database",
        "view",
        "comment",
        IndexType::Siam,
        Category::Memory,
        LevelType::Small,
        2,
    );
    create_view(
        "database",
        "view1",
        "comment",
        IndexType::Siam,
        Category::Memory,
        LevelType::Small,
        3,
    );
    put("database", "view", "md516", "database1 tValue", 1);
    get("database", "view", "md516", 1);
    put("database", "view", "md516", "database2 tValue", 2);
    get("database", "view", "md516", 2);
    set("database", "view", "md516", "database3 tValue", 3);
    get("database", "view", "md516", 3);
    modify_view("database", "view", "view_new", 4);
    get("database", "view_new", "md516", 4);
    put("database", "view_new", "md5161", "database5 tValue", 5);
    put("database", "view_new", "md5162", "database6 tValue", 6);
    put("database", "view_new", "md5163", "database7 tValue", 7);
    put("database", "view_new", "md5164", "database8 tValue", 8);
    get("database", "view_new", "md5161", 5);
    get("database", "view_new", "md5162", 7);
    get("database", "view_new", "md5163", 8);
    get("database", "view_new", "md5164", 9);
    get("database", "view_new", "md5165", 10);
    get("database", "view", "md516", 11);
    modify_database("database", "database1", 13);
    get("database1", "view_new", "md516", 12);
    get("database1", "view_new", "md516", 14);
    get("database", "view_new", "md516", 15);
}

#[test]
fn put_document() {
    create_database("database", "comment", 1);
    create_database("database", "comment", 2);
    create_database("database1", "comment", 3);
    create_view(
        "database",
        "view_doc",
        "comment",
        IndexType::Siam,
        Category::Document,
        LevelType::Small,
        1,
    );
    create_view(
        "database",
        "view_doc",
        "comment",
        IndexType::Siam,
        Category::Document,
        LevelType::Small,
        2,
    );
    create_view(
        "database",
        "view_doc1",
        "comment",
        IndexType::Siam,
        Category::Document,
        LevelType::Small,
        3,
    );
    // create_index("database", "view_doc", "index", false, 1);
    // create_index("database", "view_doc", "index", false, 2);
    put("database", "view_doc", "md516", "database1 tValue", 1);
    get("database", "view_doc", "md516", 1);
    put("database", "view_doc", "md516", "database2 tValue", 2);
    get("database", "view_doc", "md516", 2);
    set("database", "view_doc", "md516", "database3 tValue", 3);
    get("database", "view_doc", "md516", 3);
    modify_view("database", "view_doc", "view_doc_new", 4);
    get("database", "view_doc_new", "md516", 4);
    put("database", "view_doc_new", "md5161", "database5 tValue", 5);
    put("database", "view_doc_new", "md5162", "database6 tValue", 6);
    put("database", "view_doc_new", "md5163", "database7 tValue", 7);
    put("database", "view_doc_new", "md5164", "database8 tValue", 8);
    get("database", "view_doc_new", "md5161", 5);
    get("database", "view_doc_new", "md5162", 6);
    get("database", "view_doc_new", "md5163", 7);
    get("database", "view_doc_new", "md5164", 8);
    get("database", "view_doc_new", "md5165", 9);
    get("database", "view_doc", "md516", 10);
    get("database1", "view_doc_new", "md516", 11);
    modify_database("database", "database1", 1);
    modify_database("database", "database2", 2);
    get("database2", "view_doc_new", "md516", 12);
    get("database", "view_doc_new", "md516", 13);
}

#[test]
fn put_document2() {
    create_database("database_test", "comment", 1);
    create_view(
        "database_test",
        "view_test_doc_32",
        "comment",
        IndexType::Siam,
        Category::Document,
        LevelType::Small,
        1,
    );
    put("database_test", "view_test_doc_32", "key", "value1", 321);
    get("database_test", "view_test_doc_32", "key", 322);
    set("database_test", "view_test_doc_32", "key", "value2", 323);
    get("database_test", "view_test_doc_32", "key", 324);

    create_view(
        "database_test",
        "view_test_doc_64",
        "comment",
        IndexType::Siam,
        Category::Document,
        LevelType::Large,
        1,
    );

    put("database_test", "view_test_doc_64", "key", "result 1", 641);
    get("database_test", "view_test_doc_64", "key", 641);
    set("database_test", "view_test_doc_64", "key", "result 2", 642);
    get("database_test", "view_test_doc_64", "key", 642);
    put("database_test", "view_test_doc_64", "key1", "result 3", 643);
    get("database_test", "view_test_doc_64", "key1", 643);
    set("database_test", "view_test_doc_64", "key1", "result 4", 644);
    get("database_test", "view_test_doc_64", "key1", 644);
    put("database_test", "view_test_doc_64", "key2", "result 5", 645);
    get("database_test", "view_test_doc_64", "key2", 645);
    set("database_test", "view_test_doc_64", "key2", "result 6", 646);
    get("database_test", "view_test_doc_64", "key2", 646);
    put("database_test", "view_test_doc_64", "key3", "result 7", 647);
    get("database_test", "view_test_doc_64", "key3", 647);
    set("database_test", "view_test_doc_64", "key3", "result 8", 648);
    get("database_test", "view_test_doc_64", "key3", 648);
    get("database_test", "view_test_doc_64", "key11", 6423);
}

#[test]
fn db_view_index_create_test() {
    create_database("database_create_test1", "comment", 1);
    create_view(
        "database_create_test1",
        "view_test_doc_32",
        "comment",
        IndexType::Siam,
        Category::Document,
        LevelType::Small,
        1,
    );
    create_view(
        "database_create_test1",
        "view_test_doc_64",
        "comment",
        IndexType::Siam,
        Category::Document,
        LevelType::Large,
        1,
    );
    create_index(
        "database_create_test1",
        "view_test_doc_32",
        "name",
        false,
        1,
    );
    create_index("database_create_test1", "view_test_doc_64", "age", false, 1);

    create_database("database_create_test2", "comment", 1);
    create_view(
        "database_create_test2",
        "view_test_doc_32",
        "comment",
        IndexType::Siam,
        Category::Document,
        LevelType::Small,
        1,
    );
    create_view(
        "database_create_test2",
        "view_test_doc_64",
        "comment",
        IndexType::Siam,
        Category::Document,
        LevelType::Large,
        1,
    );
    create_index(
        "database_create_test2",
        "view_test_doc_32",
        "name",
        false,
        1,
    );
    create_index("database_create_test2", "view_test_doc_64", "age", false, 1);
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u8,
    blog: String,
    addr: String,
    married: bool,
    job: Job,
}

#[derive(Serialize, Deserialize)]
struct Job {
    company: String,
    age: u8,
}

#[test]
fn put_document3_index_custom() {
    create_database("database_test_index", "comment", 1);
    create_view(
        "database_test_index",
        "view_test_doc_32",
        "comment",
        IndexType::Siam,
        Category::Document,
        LevelType::Small,
        1,
    );
    create_index("database_test_index", "view_test_doc_32", "age", false, 1);
    create_index(
        "database_test_index",
        "view_test_doc_32",
        "married",
        false,
        1,
    );
    create_index("database_test_index", "view_test_doc_32", "job", false, 1);

    let user = User {
        name: "aaa".to_string(),
        age: 20,
        blog: "true".to_string(),
        addr: "ccc".to_string(),
        married: false,
        job: Job {
            company: "ddd".to_string(),
            age: 10,
        },
    };
    let user_json_str = serde_json::to_string(&user).unwrap();

    put(
        "database_test_index",
        "view_test_doc_32",
        "key",
        user_json_str.as_str(),
        1,
    );
    get("database_test_index", "view_test_doc_32", "key", 2);
    set(
        "database_test_index",
        "view_test_doc_32",
        "key",
        user_json_str.as_str(),
        3,
    );
    get("database_test_index", "view_test_doc_32", "key", 4);
}

#[derive(Serialize, Deserialize)]
struct Teacher {
    name: String,
    age: u8,
    height: u8,
    blog: String,
    married: bool,
}

#[test]
fn select_document1() {
    create_database("select_document1", "comment", 1);
    create_view(
        "select_document1",
        "view1",
        "comment",
        IndexType::Siam,
        Category::Document,
        LevelType::Small,
        1,
    );
    create_index("select_document1", "view1", "age", false, 1);
    create_index("select_document1", "view1", "job", false, 1);

    let user_str1 = serde_json::to_string(&create_t(10, 102)).unwrap();
    let user_str2 = serde_json::to_string(&create_t(15, 12)).unwrap();
    let user_str3 = serde_json::to_string(&create_t(1, 192)).unwrap();
    let user_str4 = serde_json::to_string(&create_t(7, 82)).unwrap();
    let user_str5 = serde_json::to_string(&create_t(4, 2)).unwrap();
    let user_str6 = serde_json::to_string(&create_t(9, 1)).unwrap();

    put("select_document1", "view1", "10", user_str1.as_str(), 1);
    put("select_document1", "view1", "15", user_str2.as_str(), 2);
    put("select_document1", "view1", "1", user_str3.as_str(), 3);
    put("select_document1", "view1", "7", user_str4.as_str(), 4);
    put("select_document1", "view1", "4", user_str5.as_str(), 5);
    put("select_document1", "view1", "9", user_str6.as_str(), 6);

    get("select_document1", "view1", "10", 11);
    get("select_document1", "view1", "15", 12);
    get("select_document1", "view1", "1", 13);
    get("select_document1", "view1", "7", 14);
    get("select_document1", "view1", "4", 15);
    get("select_document1", "view1", "9", 16);

    let cond_str1 = r#"
  {
    "Conditions":[
        {
            "Param":"age",
            "Cond":"le",
            "Value":9
        }
    ],
    "Sort":{
        "Param":"height",
        "Asc":false
    },
    "Skip":5,
    "Limit":30
  }"#;
    select(
        "select_document1",
        "view1",
        cond_str1.as_bytes().to_vec(),
        17,
    );

    let cond_str2 = r#"
  {
    "Conditions":[
        {
            "Param":"age",
            "Cond":"gt",
            "Value":3
        }
    ],
    "Sort":{
        "Param":"height",
        "Asc":true
    },
    "Skip":5,
    "Limit":30
  }"#;
    select(
        "select_document1",
        "view1",
        cond_str2.as_bytes().to_vec(),
        18,
    );
}

fn create_t(a: u8, h: u8) -> Teacher {
    Teacher {
        name: a.to_string(),
        age: a,
        height: h,
        blog: a.to_string(),
        married: a % 2 == 0,
    }
}

fn create_database(database_name: &str, database_comment: &str, position: usize) {
    create_database_string(
        database_name.to_string(),
        database_comment.to_string(),
        position,
    )
}

fn create_database_string(database_name: String, database_comment: String, position: usize) {
    match GLOBAL_CLIENT.create_database(database_name, database_comment) {
        Err(err) => println!("create_database{} database_test = {}", position, err),
        _ => {}
    }
}

fn create_view(
    database_name: &str,
    view_name: &str,
    view_comment: &str,
    index_type: IndexType,
    view_category: Category,
    view_level: LevelType,
    position: usize,
) {
    match GLOBAL_CLIENT.create_view(
        database_name.to_string(),
        view_name.to_string(),
        view_comment.to_string(),
        index_type,
        view_category,
        view_level,
    ) {
        Err(err) => println!("create_view{} view_test_doc = {}", position, err),
        _ => {}
    }
}

fn create_index(
    database_name: &str,
    view_name: &str,
    key_structure: &str,
    primary: bool,
    position: usize,
) {
    match GLOBAL_CLIENT.create_index(
        database_name.to_string(),
        view_name.to_string(),
        key_structure.to_string(),
        primary,
    ) {
        Err(err) => println!("create_index{} {} = {}", position, key_structure, err),
        _ => {}
    }
}

fn put(database_name: &str, view_name: &str, key: &str, value: &str, position: usize) {
    match GLOBAL_CLIENT.put(
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
    match GLOBAL_CLIENT.set(
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
    match GLOBAL_CLIENT.get(
        database_name.to_string(),
        view_name.to_string(),
        key.to_string(),
    ) {
        Ok(vu8) => println!(
            "get{} is {:#?}",
            position,
            String::from_utf8(vu8).unwrap().as_str()
        ),
        Err(ie) => println!("get{} is {:#?}", position, ie.source().unwrap().to_string()),
    }
}

fn select(database_name: &str, view_name: &str, constraint_json_bytes: Vec<u8>, position: usize) {
    match GLOBAL_CLIENT.select(
        database_name.to_string(),
        view_name.to_string(),
        constraint_json_bytes,
    ) {
        Ok(e) => {
            println!(
                "select{},count={},index_name={}",
                position, e.count, e.index_name
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

fn modify_view(database_name: &str, view_old_name: &str, view_new_name: &str, position: usize) {
    match GLOBAL_CLIENT.modify_view(
        database_name.to_string(),
        view_old_name.to_string(),
        view_new_name.to_string(),
    ) {
        Err(ie) => println!(
            "modify view {} is {:#?}",
            position,
            ie.source().unwrap().to_string()
        ),
        _ => {}
    }
}

fn modify_database(database_old_name: &str, database_new_name: &str, position: usize) {
    match GLOBAL_CLIENT
        .modify_database(database_old_name.to_string(), database_new_name.to_string())
    {
        Err(ie) => println!(
            "modify database {} is {:#?}",
            position,
            ie.source().unwrap().to_string()
        ),
        _ => {}
    }
}
