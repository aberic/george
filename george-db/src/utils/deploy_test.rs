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
mod test {
    use crate::utils::deploy::GLOBAL_CONFIG;
    use george_deploy::ConfigDB;

    #[test]
    fn init_test() {
        let conf = GLOBAL_CONFIG
            .write()
            .unwrap()
            .init(ConfigDB::new("george-db/src/test/george".to_string(), 100));
        println!("conf = {:#?}", conf);
        println!("GLOBAL_CONFIG = {:#?}", GLOBAL_CONFIG);
    }
}
