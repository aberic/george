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

    use crate::json::{JsonExec, JsonGet, JsonHandler, JsonNew};
    use crate::{Json, JsonArray};

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
    const GET: &str = r#"
                        {
                            "string": "text",
                            "u64": 127,
                            "i64": -128,
                            "f64": 549.127,
                            "bool": false,
                            "object": {
                                          "string": "text",
                                          "u64": 127,
                                          "i64": -128,
                                          "f64": 549.127,
                                          "bool": false
                                       }
                        }"#;
    const ARRAY: &str = r#"
                        {
                            "string": "text",
                            "u64": 127,
                            "i64": -128,
                            "f64": 549.127,
                            "bool": false,
                            "object": {
                                          "string": "text",
                                          "u64": 127,
                                          "i64": -128,
                                          "f64": 549.127,
                                          "bool": false
                             },
                            "array1": [
                                {
                                    "string": "text",
                                    "u64": 127,
                                    "i64": -128,
                                    "f64": 549.127,
                                    "bool": false,
                                    "array": ["hello", "world", "test"]
                                },
                                {
                                    "string": "text",
                                    "u64": 127,
                                    "i64": -128,
                                    "f64": 549.127,
                                    "bool": false,
                                    "array": [1, 100, 10000]
                                },
                                {
                                    "string": "text",
                                    "u64": 127,
                                    "i64": -128,
                                    "f64": 549.127,
                                    "bool": false,
                                    "array": [5.4, 100.1, 10000.98]
                                }
                            ],
                            "array2": ["one", "two", { "three": "object" }]
                        }"#;
    const ARRAYS: &str = r#"
                        [
                            {
                                "string": "text",
                                "u64": 127,
                                "i64": -128,
                                "f64": 549.127,
                                "bool": false,
                                "array": ["hello", "world", "test"]
                            },
                            {
                                "string": "text",
                                "u64": 127,
                                "i64": -128,
                                "f64": 549.127,
                                "bool": false,
                                "array": [1, 100, 10000]
                            },
                            {
                                "string": "text",
                                "u64": 127,
                                "i64": -128,
                                "f64": 549.127,
                                "bool": false,
                                "array": [5.4, 100.1, 10000.98]
                            },
                            {
                                "string": "text",
                                "u64": 127,
                                "i64": -128,
                                "f64": 549.127,
                                "bool": false,
                                "array": [5.4, "test", 10000, false, -99]
                            }
                        ]
                        "#;
    const ARRAY_OBJECT: &str = r#"
                        [
                            {
                                "name": "琼台博客",
                                "age": 30,
                                "blog": "https://www.qttc.net",
                                "addr": "4114 Sepulveda Blvd"
                            },
                            {
                                "name": "琼台博客",
                                "age": 30,
                                "blog": "https://www.qttc.net",
                                "addr": "4114 Sepulveda Blvd"
                            }
                        ]
                        "#;

    #[test]
    fn test_self() {
        let json1 = Json::new(DATA).unwrap();
        let json2 = Json::new(DATA.to_string()).unwrap();
        let json3 = Json::new(DATA.as_bytes()).unwrap();
        let json4 = Json::new(DATA.as_bytes().to_vec()).unwrap();
        println!("json1 to string = {}", json1.to_string());
        println!("json2 to string = {}", json2.to_string());
        println!("json3 to string = {}", json3.to_string());
        println!("json4 to string = {}", json4.to_string());
        println!("json1 to slice = {:#?}", String::from_utf8(json1.to_vec()))
    }

    #[test]
    fn test_obj() {
        let json = Json::new(USER).unwrap();
        let user: User = json.to_object().unwrap();
        println!("user = {:#?}", user);
        let u1: User = Json::string_2_obj(json.to_string().as_str()).unwrap();
        println!("user = {:#?}", u1);
        let u2: User = Json::bytes_2_obj(json.to_vec().as_slice()).unwrap();
        println!("user = {:#?}", u2);
        let u3: User = Json::value_2_obj(json.value()).unwrap();
        println!("user = {:#?}", u3);
    }

    #[test]
    fn test_object_exec() {
        let json = Json::new(GET).unwrap();
        println!("string = {}", json.get_string("string").unwrap());
        println!("u64 = {}", json.get_u64("u64").unwrap());
        println!("i64 = {}", json.get_i64("i64").unwrap());
        println!("f64 = {}", json.get_f64("f64").unwrap());
        println!("bool = {}", json.get_bool("bool").unwrap());
        println!();
        println!("string = {}", json.is_string("string"));
        println!("u64 = {}", json.is_u64("u64"));
        println!("i64 = {}", json.is_i64("i64"));
        println!("f64 = {}", json.is_f64("f64"));
        println!("bool = {}", json.is_bool("bool"));
        println!();
        println!("string = {}", json.is_u64("string"));
        println!("u64 = {}", json.is_i64("u64"));
        println!("i64 = {}", json.is_f64("i64"));
        println!("f64 = {}", json.is_bool("f64"));
        println!("bool = {}", json.is_string("bool"));
        println!();
        let object = json.get_object("object").unwrap();
        println!("object string = {}", object.get_string("string").unwrap());
        println!("object u64 = {}", object.get_u64("u64").unwrap());
        println!("object i64 = {}", object.get_i64("i64").unwrap());
        println!("object f64 = {}", object.get_f64("f64").unwrap());
        println!("object bool = {}", object.get_bool("bool").unwrap());
    }

    #[test]
    fn test_array_self() {
        let array1 = Json::new(ARRAYS).unwrap();
        let array2 = Json::new(ARRAYS.to_string()).unwrap();
        let array3 = Json::new(ARRAYS.as_bytes()).unwrap();
        let array4 = Json::new(ARRAYS.as_bytes().to_vec()).unwrap();
        println!("array1 to string = {}", array1.to_string());
        println!("array2 to string = {}", array2.to_string());
        println!("array3 to string = {}", array3.to_string());
        println!("array4 to string = {}", array4.to_string());
        println!(
            "array1 to slice = {:#?}",
            String::from_utf8(array1.to_vec())
        )
    }

    #[test]
    fn test_array_obj() {
        let array = JsonArray::new(ARRAY_OBJECT).unwrap();
        let users: Vec<User> = array.to_object().unwrap();
        println!("user = {:#?}", users);
    }

    #[test]
    fn test_array1() {
        let json = Json::new(ARRAY).unwrap();
        println!("string = {}", json.get_string("string").unwrap());
        println!("u64 = {}", json.get_u64("u64").unwrap());
        println!("i64 = {}", json.get_i64("i64").unwrap());
        println!("f64 = {}", json.get_f64("f64").unwrap());
        println!("bool = {}", json.get_bool("bool").unwrap());
        let array = json.get_array("array1").unwrap();
        let object = array.get_object(0).unwrap();
        println!("object string = {}", object.get_string("string").unwrap());
        println!("object u64 = {}", object.get_u64("u64").unwrap());
        println!("object i64 = {}", object.get_i64("i64").unwrap());
        println!("object f64 = {}", object.get_f64("f64").unwrap());
        println!("object bool = {}", object.get_bool("bool").unwrap());
        let array = object.get_array("array").unwrap();
        println!("array 0 = {}", array.get_string(0).unwrap());
    }

    #[test]
    fn test_array2() {
        let array = JsonArray::new(ARRAYS).unwrap();
        let json = array.get_object(0).unwrap();
        println!("string = {}", json.get_string("string").unwrap());
        println!("u64 = {}", json.get_u64("u64").unwrap());
        println!("i64 = {}", json.get_i64("i64").unwrap());
        println!("f64 = {}", json.get_f64("f64").unwrap());
        println!("bool = {}", json.get_bool("bool").unwrap());
        let array = json.get_array("array").unwrap();
        println!("array 0 = {}", array.get_string(0).unwrap());
    }

    #[test]
    fn test_array3() {
        let array = JsonArray::new(ARRAYS).unwrap();
        let json = array.get_object(3).unwrap();
        println!("string = {}", json.get_string("string").unwrap());
        println!("u64 = {}", json.get_u64("u64").unwrap());
        println!("i64 = {}", json.get_i64("i64").unwrap());
        println!("f64 = {}", json.get_f64("f64").unwrap());
        println!("bool = {}", json.get_bool("bool").unwrap());
        let array = json.get_array("array").unwrap();
        println!("array 0 = {}", array.get_f64(0).unwrap());
        println!("array 0 = {}", array.get_string(1).unwrap());
        println!("array 0 = {}", array.get_u64(2).unwrap());
        println!("array 0 = {}", array.get_bool(3).unwrap());
        println!("array 0 = {}", array.get_i64(4).unwrap());
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
            String::from_utf8(Json::obj_2_bytes(&user).unwrap())
        );
        println!("object = {}", Json::object(&user).unwrap().to_string());
    }
}
