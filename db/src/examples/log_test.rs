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

    use log::LevelFilter;
    use logs::LogModule;

    #[test]
    fn logs() {
        let module = LogModule {
            name: String::from("db"),
            pkg: "".to_string(),
            level: LevelFilter::Trace,
            additive: false,
            dir: String::from("src/test"),
            file_max_size: 1024,
            file_max_count: 7,
        };
        module.set_log(vec![LogModule {
            name: "mod1".to_string(),
            pkg: "logs::examples::log_test::log_test_mod".to_string(),
            level: LevelFilter::Trace,
            additive: true,
            dir: "".to_string(),
            file_max_size: 0,
            file_max_count: 0,
        }]);
        log::trace!("Hello, world!");
        log::debug!("Hello, world!");
        log::info!("Hello, world!");
        log::warn!("Hello, world!");
        log::error!("Hello, world!");
    }
}
