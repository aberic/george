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
mod reader {
    use crate::io::reader::{read_all, read_sub_bytes};

    #[test]
    fn reader_test() {
        let s = read_all("src/examples/conf.yaml");
        println!("s = {:#?}", s);
    }

    #[test]
    fn read_sub_bytes_test() {
        println!(
            "res1 = {:#?}",
            read_sub_bytes("src/examples/29f459a44fee58c7.gge".to_string(), 448, 8,)
        );
        println!(
            "res2 = {:#?}",
            read_sub_bytes("src/examples/29f459a44fee58c7.gge".to_string(), 0, 2048,)
        );
    }
}
