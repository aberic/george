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
    use crate::io::file::create_file_str;
    use crate::io::writer::{write_append_bytes, write_seek_u8s};

    #[test]
    fn reader_test() {
        create_file_str("src/test/file/g.txt", true).unwrap();
        match write_append_bytes(
            "src/test/file/g.txt".to_string(),
            vec![
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
            ],
        ) {
            Ok(()) => {
                let vs: Vec<u8> = vec![0x0b, 0x0c, 0x0d, 0x0e];
                match write_seek_u8s("src/test/file/g.txt".to_string(), 3, vs.as_slice()) {
                    Err(err) => println!("err = {}", err),
                    _ => {}
                }
            }
            Err(err) => println!("err = {}", err),
        }
    }
}
