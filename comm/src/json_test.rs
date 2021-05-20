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

#[cfg(test)]
mod json {
    use serde::{Deserialize, Serialize};

    use crate::json::{Json, JsonFrom};

    #[derive(Debug, Serialize, Deserialize)]
    struct User {
        name: String,
        age: u8,
        blog: String,
        addr: String,
    }

    const DATA: &str = r#"
                        {
                            "name": "John Doe",
                            "age": 43,
                            "phones": [
                                "+44 1234567",
                                "+44 2345678"
                            ]
                        }"#;
    const USER: &str = r#"
                        {
                            "name": "琼台博客",
                            "age": 30,
                            "blog": "https://www.qttc.net",
                            "addr": "4114 Sepulveda Blvd"
                        }"#;

    #[test]
    fn test_self() {
        let json1 = Json::from_string(DATA).unwrap();
        let json2 = Json::from_string(DATA.to_string()).unwrap();
        let json3 = Json::from_slice(DATA.as_bytes()).unwrap();
        let json4 = Json::from_slice(DATA.as_bytes().to_vec()).unwrap();
        println!("json1 to string = {}", json1.to_string());
        println!("json2 to string = {}", json2.to_string());
        println!("json3 to string = {}", json3.to_string());
        println!("json4 to string = {}", json4.to_string());
        println!("json1 to slice = {:#?}", String::from_utf8(json1.to_vec()))
    }

    #[test]
    fn test_obj() {
        let json = Json::from_string(USER).unwrap();
        let user: User = json.to_obj().unwrap();
        println!("user = {:#?}", user)
    }

    #[test]
    fn test_out() {
        let user = User {
            name: "1".to_string(),
            age: 2,
            blog: "3".to_string(),
            addr: "4".to_string(),
        };
        println!("object to string = {}", Json::obj_2_string(&user).unwrap());
        println!(
            "object to string = {:#?}",
            String::from_utf8(Json::obj_2_vec(&user).unwrap())
        );
    }
}
