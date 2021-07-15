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

use std::path::Path;

use log::LevelFilter;

use george_comm::errors::GeorgeResult;
use george_log::LogModule;

use crate::{Conf, Config, ConfigDB, ConfigLog, ConfigServer, Init, LogPolicy};

impl Init {
    pub fn from<P: AsRef<Path>>(filepath: P) -> GeorgeResult<Self> {
        let conf = Conf::from(filepath)?;
        let log_main = log_module_main(conf.log_unwrap());
        log_main.set_log(vec![]);
        Ok(Init { conf, log_main })
    }

    pub fn config(&self) -> Option<Config> {
        self.conf.config()
    }

    pub fn log(&self) -> Option<ConfigLog> {
        self.conf.log()
    }

    pub fn db(&self) -> Option<ConfigDB> {
        self.conf.db()
    }

    pub fn server(&self) -> Option<ConfigServer> {
        self.conf.server()
    }

    pub fn log_dir(&self) -> Option<String> {
        self.conf.log()?.log_dir
    }

    pub fn port_unwrap(&self) -> u16 {
        self.conf
            .server()
            .unwrap()
            .port
            .expect("It's not gonna happen!")
    }

    pub fn tls(&self) -> bool {
        match self.conf.server() {
            Some(res) => match res.tls {
                Some(_) => true,
                None => false,
            },
            None => false,
        }
    }

    pub fn rustls(&self) -> bool {
        match self.conf.server().unwrap().tls {
            Some(res) => match res.rust_tls {
                Some(res) => res,
                None => false,
            },
            None => true,
        }
    }

    pub fn tls_key_unwrap(&self) -> String {
        self.conf.server().unwrap().tls.unwrap().key.unwrap()
    }

    pub fn tls_cert_unwrap(&self) -> String {
        self.conf.server().unwrap().tls.unwrap().cert.unwrap()
    }

    pub fn tls_ca(&self) -> Option<String> {
        self.conf.server().unwrap().tls.unwrap().ca
    }

    pub fn domain(&self) -> String {
        match self.conf.server().unwrap().tls.unwrap().domain {
            Some(res) => res,
            None => "".to_string(),
        }
    }

    pub fn timeout(&self) -> Option<u64> {
        self.conf.server()?.http?.timeout
    }

    pub fn concurrency_limit_per_connection(&self) -> Option<usize> {
        self.conf.server()?.http?.concurrency_limit_per_connection
    }

    pub fn tcp_nodelay(&self) -> Option<bool> {
        self.conf.server()?.http?.tcp_nodelay
    }

    pub fn tcp_keepalive(&self) -> Option<u64> {
        self.conf.server()?.http?.tcp_keepalive
    }

    pub fn http2_keepalive_interval(&self) -> Option<u64> {
        self.conf.server()?.http?.http2_keepalive_interval
    }

    pub fn http2_keepalive_timeout(&self) -> Option<u64> {
        self.conf.server()?.http?.http2_keepalive_timeout
    }

    pub fn initial_connection_window_size(&self) -> Option<u32> {
        self.conf.server()?.http?.initial_connection_window_size
    }

    pub fn initial_stream_window_size(&self) -> Option<u32> {
        self.conf.server()?.http?.initial_stream_window_size
    }

    pub fn max_concurrent_streams(&self) -> Option<u32> {
        self.conf.server()?.http?.max_concurrent_streams
    }

    pub fn max_frame_size(&self) -> Option<u32> {
        self.conf.server()?.http?.max_frame_size
    }

    pub fn db_unwrap(&self) -> ConfigDB {
        self.conf.db().expect("It's not gonna happen!")
    }

    pub fn log_dir_unwrap(&self) -> String {
        self.conf
            .log_unwrap()
            .log_dir
            .expect("It's not gonna happen!")
    }

    pub fn add_log_policy(&self, log_policy: LogPolicy) {
        add_log(self.log_main.clone(), log_policy)
    }
}

fn add_log(log_main: LogModule, log_policy: LogPolicy) {
    let module_record = LogModule {
        name: log_policy.name,
        pkg: log_policy.pkg,
        level: log_main.level,
        additive: log_policy.additive,
        dir: log_policy.dir,
        file_max_size: log_main.file_max_size,
        file_max_count: log_main.file_max_count,
    };
    log_main.set_log(vec![module_record]);
}

fn log_module_main(config_log: ConfigLog) -> LogModule {
    LogModule {
        name: String::from("george-server"),
        pkg: "".to_string(),
        level: match config_log.log_level {
            Some(level) => log_level(level),
            None => LevelFilter::Debug,
        },
        additive: match config_log.additive {
            Some(res) => res,
            None => false,
        },
        dir: match config_log.log_dir {
            Some(res) => res,
            None => "george-server/src/test/george".to_string(),
        },
        file_max_size: match config_log.log_file_max_size {
            Some(res) => res,
            None => 1,
        },
        file_max_count: match config_log.log_file_max_count {
            Some(res) => res,
            None => 5,
        },
    }
}

fn log_level(level: String) -> LevelFilter {
    match level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Off,
    }
}
