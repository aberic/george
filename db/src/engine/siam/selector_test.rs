use crate::engine::siam::selector::Selector;
use crate::engine::traits::TIndex;
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
    let selector1 = Selector::new(cond_str1.as_bytes().to_vec(), indexes.clone(), true);
    println!("selector1 = {:#?}", selector1);
    let selector2 = Selector::new(cond_str2.as_bytes().to_vec(), indexes.clone(), false);
    println!("selector2 = {:#?}", selector2);
    let selector3 = Selector::new(cond_str3.as_bytes().to_vec(), indexes.clone(), true);
    println!("selector3 = {:#?}", selector3);
}
