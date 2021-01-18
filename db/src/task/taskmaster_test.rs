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

use crate::task::taskmaster::GLOBAL_MASTER;
use chrono::{Duration, NaiveDateTime};

#[test]
fn database_map_test() {
    database_map();
}

#[test]
fn base_test() {
    // database_create_test
    create_database("database_create_base_test");
    // database_modify_test
    create_database("database_modify_base_test1");
    modify_database("database_modify_base_test1", "database_modify_base_test2");
    modify_database("database_modify_base_test1", "database_modify_base_test2");
    // view_create_test
    create_view("database_view_create_base_test", "view_create_base_test");
    // view_modify_test
    create_view("database_view_modify_base_test", "view_modify_base_test1");
    modify_view(
        "database_view_modify_base_test",
        "view_modify_base_test1",
        "view_modify_base_test2",
    );
    modify_view(
        "database_view_modify_base_test",
        "view_modify_base_test1",
        "view_modify_base_test2",
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
            )
        }
    }
}

fn create_database(name: &str) {
    match GLOBAL_MASTER.create_database(String::from(name), String::from("comment")) {
        Ok(()) => println!("create database {}", name),
        Err(err) => println!("create database {} error, {}", name, err),
    }
}

fn modify_database(name: &str, old_name: &str) {
    match GLOBAL_MASTER.modify_database(String::from(name), String::from(old_name)) {
        Ok(()) => println!("modify database {} to {}", name, old_name),
        Err(err) => println!("modify database {} to {} error, {}", name, old_name, err),
    }
}

fn create_view(database_name: &str, view_name: &str) {
    match GLOBAL_MASTER.create_database(String::from(database_name), String::from("comment")) {
        Ok(()) => println!("create database {}", database_name),
        Err(err) => println!("create database {} error, {}", database_name, err),
    }
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
