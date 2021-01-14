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

use crate::engine::siam::doc::node::Node;
use crate::engine::siam::index::Index;
use crate::engine::siam::selector::Selector;
use crate::engine::traits::TIndex;
use crate::utils::comm::{Category, IndexMold};
use comm::cryptos::hash::md516;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[test]
fn new_test() {
    let cond_str1 = r#"
  {
    "Conditions":[
        {
            "Param":"Age",
            "Cond":"gt",
            "Value":3
        }
    ],
    "Sort":{
        "Param":"Age",
        "Asc":false
    },
    "Skip":5,
    "Limit":30
  }"#;

    let cond_str2 = r#"
  {
    "Sort":{
        "Param":"Age",
        "Asc":false
    },
    "Skip":5,
    "Limit":30
  }"#;

    let cond_str3 = r#"
  {
    "Limit":30
  }"#;
    let indexes: Arc<RwLock<HashMap<String, Arc<RwLock<dyn TIndex>>>>> =
        Arc::new(Default::default());
    let selector1 = Selector::run(cond_str1.as_bytes().to_vec(), indexes.clone(), true).unwrap();
    println!("selector1 = {:#?}", selector1);
    let selector2 = Selector::run(cond_str2.as_bytes().to_vec(), indexes.clone(), false).unwrap();
    println!("selector2 = {:#?}", selector2);
    let selector3 = Selector::run(cond_str3.as_bytes().to_vec(), indexes.clone(), true).unwrap();
    println!("selector3 = {:#?}", selector3);
}

#[test]
fn run_test() {
    let cond_str1 = r#"
  {
    "Conditions":[
        {
            "Param":"Age",
            "Cond":"gt",
            "Value":3
        }
    ],
    "Sort":{
        "Param":"Age",
        "Asc":false
    },
    "Skip":5,
    "Limit":30
  }"#;

    let cond_str2 = r#"
  {
    "Sort":{
        "Param":"Age",
        "Asc":false
    },
    "Skip":5,
    "Limit":30
  }"#;

    let cond_str3 = r#"
  {
    "Limit":30
  }"#;
    let indexes: Arc<RwLock<HashMap<String, Arc<RwLock<dyn TIndex>>>>> =
        Arc::new(Default::default());
    let database_id = String::from("database_id");
    let view_id = String::from("view_id");
    let index_name = String::from("Age");
    let index_id = md516(index_name.clone());
    let index = Index::create(
        database_id.clone(),
        view_id.clone(),
        index_id.clone(),
        index_name,
        false,
        Node::create_root(database_id, view_id, index_id.clone()),
        Category::Memory,
        IndexMold::String,
    );
    indexes
        .clone()
        .write()
        .unwrap()
        .insert(index_id, Arc::new(RwLock::new(index)));
    let exp = Selector::run(cond_str1.as_bytes().to_vec(), indexes.clone(), true).unwrap();
    println!("selector1 run exp = {:#?}", exp);
    // let selector2 = Selector::new(cond_str2.as_bytes().to_vec(), indexes.clone(), false);
    // let selector3 = Selector::new(cond_str3.as_bytes().to_vec(), indexes.clone(), true);
}
