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
mod seed_test {
    use crate::engine::siam::doc32::seed::Seed;
    use crate::engine::traits::TSeed;

    #[test]
    fn create_and_modify() {
        println!("Hello, world!");
        let l = Seed::create(
            "database_id".to_string(),
            "view_id".to_string(),
            "".as_bytes().to_vec(),
        );
        println!("seed is {:#?}", l);
        println!("seed md516_key = {}", l.key());
        println!();
        println!("seZed is {:#?}", l);
        println!("seed md516_key = {}", l.key());
    }
}
