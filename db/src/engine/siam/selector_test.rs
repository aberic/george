use crate::engine::siam::document::node::Node;
use crate::engine::siam::index::Index;
use crate::engine::siam::selector::Selector;
use crate::engine::traits::TIndex;
use crate::utils::comm::{Category, LevelType};
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
    let selector1 = Selector::new(cond_str1.as_bytes().to_vec(), indexes.clone(), true).unwrap();
    println!("selector1 = {:#?}", selector1);
    let selector2 = Selector::new(cond_str2.as_bytes().to_vec(), indexes.clone(), false).unwrap();
    println!("selector2 = {:#?}", selector2);
    let selector3 = Selector::new(cond_str3.as_bytes().to_vec(), indexes.clone(), true).unwrap();
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
    let key_structure = String::from("Age");
    let index_id = md516(key_structure.clone());
    let index = Index::create(
        database_id.clone(),
        view_id.clone(),
        index_id.clone(),
        key_structure,
        false,
        Node::create_root(database_id, view_id, index_id.clone(), LevelType::Large),
        Category::Memory,
        LevelType::Large,
    );
    indexes
        .clone()
        .write()
        .unwrap()
        .insert(index_id, Arc::new(RwLock::new(index)));
    let selector1 = Selector::new(cond_str1.as_bytes().to_vec(), indexes.clone(), true).unwrap();
    let exp = selector1.run();
    println!("selector1 run exp = {:#?}", exp);
    // let selector2 = Selector::new(cond_str2.as_bytes().to_vec(), indexes.clone(), false);
    // let selector3 = Selector::new(cond_str3.as_bytes().to_vec(), indexes.clone(), true);
}
