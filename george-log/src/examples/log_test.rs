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
mod log_test {
    use crate::examples::log_test::log_test_mod1::logs_mod1;
    use crate::examples::log_test::log_test_mod2::logs_mod2;
    use crate::LogModule;
    use log::LevelFilter;

    #[test]
    fn logs() {
        let module = LogModule {
            name: String::from("set"),
            pkg: "".to_string(),
            level: LevelFilter::Debug,
            additive: true,
            dir: String::from("src/test"),
            file_max_size: 1024,
            file_max_count: 7,
        };
        module.set_log(vec![
            LogModule {
                name: "mod1".to_string(),
                pkg: "george-log::examples::log_test::log_test_mod1".to_string(),
                level: LevelFilter::Trace,
                additive: true,
                dir: String::from("src/test"),
                file_max_size: 1024,
                file_max_count: 7,
            },
            LogModule {
                name: "mod2".to_string(),
                pkg: "george-log::examples::log_test::log_test_mod2".to_string(),
                level: LevelFilter::Debug,
                additive: true,
                dir: String::from(""),
                file_max_size: 0,
                file_max_count: 0,
            },
        ]);
        log::debug!("Hello, world!");
        log::info!("Hello, world!");
        log::warn!("Hello, world!");
        log::error!("Hello, world!");

        logs_mod1();
        logs_mod2();
    }
}

#[cfg(test)]
mod log_test_mod1 {
    pub fn logs_mod1() {
        log::trace!("Hello, world! logs_mod");
        log::debug!("Hello, world! logs_mod");
        log::info!("Hello, world! logs_mod");
        log::warn!("Hello, world! logs_mod");
        log::error!("Hello, world! logs_mod");
    }
}

#[cfg(test)]
mod log_test_mod2 {
    pub fn logs_mod2() {
        log::trace!("Hello, world! logs_mod");
        log::debug!("Hello, world! logs_mod");
        log::info!("Hello, world! logs_mod");
        log::warn!("Hello, world! logs_mod");
        log::error!("Hello, world! logs_mod");
    }
}
