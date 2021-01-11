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
    use crate::set_log;

    #[test]
    fn logs() {
        set_log(
            String::from("log"),
            String::from("src/test"),
            1024,
            7,
            String::from("trace"),
        );
        log::trace!("Hello, world!");
        log::debug!("Hello, world!");
        log::info!("Hello, world!");
        log::warn!("Hello, world!");
        log::error!("Hello, world!");
    }
}
