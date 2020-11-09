use serde::{Deserialize, Serialize};

use crate::utils::comm::key_fetch;

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
fn key_fetch_test() {
    let user = User {
        name: "a".to_string(),
        age: 10,
        blog: "false".to_string(),
        addr: "c".to_string(),
        married: false,
        job: Job {
            company: "d".to_string(),
            age: 20,
        },
    };
    let user_json_str = serde_json::to_string(&user).unwrap();
    let user_byte = user_json_str.into_bytes();
    println!(
        "res1 = {:#?}",
        key_fetch(String::from("name"), user_byte.clone())
    );
    println!(
        "res2 = {:#?}",
        key_fetch(String::from("age"), user_byte.clone())
    );
    println!(
        "res3 = {:#?}",
        key_fetch(String::from("blog"), user_byte.clone())
    );
    println!(
        "res4 = {:#?}",
        key_fetch(String::from("married"), user_byte.clone())
    );
    println!(
        "res4 = {:#?}",
        key_fetch(String::from("job"), user_byte.clone())
    );
}
