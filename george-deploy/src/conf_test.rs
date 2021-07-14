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
    use crate::{Conf, Init, LogPolicy};

    #[test]
    fn init_test() {
        let mut conf = Conf::from("src/example/conf.yaml").unwrap();
        conf.check();
        println!("conf = {:#?}", conf);

        let mut conf = Conf::from("src/example/conf_empty.yaml").unwrap();
        conf.check();
        println!("conf_empty = {:#?}", conf);

        let mut conf = Conf::from("src/example/conf_with_no_db.yaml").unwrap();
        conf.check();
        println!("conf_with_no_db = {:#?}", conf);

        let mut conf = Conf::from("src/example/conf_with_no_log.yaml").unwrap();
        conf.check();
        println!("conf_with_no_log = {:#?}", conf);

        let mut conf = Conf::from("src/example/conf_with_no_server.yaml").unwrap();
        conf.check();
        println!("conf_with_no_server = {:#?}", conf);

        let mut conf = Conf::from("src/example/conf_tls.yaml").unwrap();
        conf.check();
        println!("conf_tls = {:#?}", conf);
    }

    #[test]
    fn test() {
        let init = Init::from("src/example/conf.yaml").unwrap();
        init.add_log_policy(LogPolicy::new(
            format!("{}/{}", init.log_main.dir, "records"),
            "test".to_string(),
            "george-deploy".to_string(),
            true,
        ));
        log::debug!("test");
        log::info!("test");
    }
}
