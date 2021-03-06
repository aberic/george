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

use crate::tools::Trim;

impl Trim {
    pub fn str(str: String) -> String {
        trim_str(str)
    }

    pub fn parse(str: String) -> String {
        let str = trim_str(str);
        str[0..str.len() - 1].to_string()
    }

    pub fn split(str: String) -> Vec<String> {
        let mut vss: Vec<String> = vec![];
        let mut vsi = str.split(" ");
        let mut v = vsi.next();
        while v.is_some() {
            vss.push(v.unwrap().to_string());
            v = vsi.next()
        }
        vss
    }
}

fn trim_str(str: String) -> String {
    let str = str.to_lowercase();
    trim_parse(str)
}

fn trim_parse(str: String) -> String {
    let str = trim_n(str);
    let str = trim_t(str);
    match str.strip_suffix(" ") {
        Some(str) => match str.strip_prefix(" ") {
            Some(str) => str.to_string(),
            None => str.to_string(),
        },
        None => match str.strip_suffix(" ") {
            Some(str) => str.to_string(),
            None => str.to_string(),
        },
    }
}

fn trim_n(str: String) -> String {
    if str.contains("\n") {
        let str = str.replace("\n", " ");
        trim_n(str)
    } else {
        str.to_string()
    }
}

fn trim_t(str: String) -> String {
    if str.contains("  ") {
        let str = str.replace("  ", " ");
        trim_t(str)
    } else {
        str.to_string()
    }
}
