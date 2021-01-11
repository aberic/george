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
    use crate::engine::siam::mem::seed::Seed;
    use crate::engine::traits::TSeed;

    #[test]
    fn create_and_modify() {
        println!("Hello, world!");
        let mut l = Seed::create(String::from("tKey"));
        // println!("seed is {:?}", l);
        // println!("seed is {:#?}", l);
        println!("seed is {:#?}", l);
        println!("seed key = {}", l.key());
        println!("seed value = {:#?}", l.value());
        println!();
        l.modify(String::from("tValue2").into_bytes());
        println!("seed is {:#?}", l);
        println!("seed key = {}", l.key());
        println!(
            "seed value = {}",
            String::from_utf8(l.value().unwrap().clone())
                .unwrap()
                .as_str()
        );
    }
}
