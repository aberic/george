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

use crate::cmd::Scan;
use crate::service::Parse;
use std::io;
use std::io::Write;

impl Scan {
    pub fn run(parse: &Parse) {
        print!("george->: ");
        io::stdout().flush().unwrap();
        let mut new_str = String::new();
        let mut all_str = String::new();
        while io::stdin().read_line(&mut new_str).is_ok() {
            if new_str.contains(";") {
                all_str.push_str(new_str.as_str());
                match parse.scan(all_str.clone()) {
                    Ok(res) => match String::from_utf8(res) {
                        Ok(res) => {
                            print!("{}", res);
                            io::stdout().flush().unwrap();
                        }
                        Err(err) => println!("error: {}", err),
                    },
                    Err(err) => println!("error: {}", err),
                }
                print!("george->: ");
                io::stdout().flush().unwrap();
                all_str.clear();
            } else {
                all_str.push_str(new_str.as_str());
            }
            new_str.clear();
        }
    }
}
