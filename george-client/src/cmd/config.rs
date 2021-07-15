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

use george_comm::errors::{Errs, GeorgeResult};
use george_comm::io::file::{FilerHandler, FilerReader};
use george_comm::io::Filer;
use george_deploy::{ConfigServerHttp, ConfigServerTLS};
use george_rpc::client::{RequestCond, TLSType};

use crate::cmd::Config;

impl Config {
    pub(crate) fn from<P: AsRef<Path>>(filepath: P) -> GeorgeResult<Self> {
        match Filer::read_bytes(&filepath) {
            Ok(conf_yaml_bytes) => match serde_yaml::from_slice(&conf_yaml_bytes) {
                Ok(res) => Ok(res),
                Err(err) => Err(Errs::strs("serde yaml", err)),
            },
            _ => Err(Errs::string(format!(
                "No config file match in path {}",
                Filer::absolute(filepath).unwrap()
            ))),
        }
    }

    pub(crate) fn tls_type(&self) -> TLSType {
        match self.config.unwrap().tls.unwrap().rust_tls {
            Some(res) => {
                if res {
                    TLSType::Rustls
                } else {
                    TLSType::Openssl
                }
            }
            None => TLSType::Openssl,
        }
    }

    pub(crate) fn tls(&self) -> bool {
        match self.config.clone() {
            Some(res) => match res.tls {
                Some(_) => true,
                None => false,
            },
            None => false,
        }
    }

    pub(crate) fn tls_ca(&self) -> Option<String> {
        self.config?.tls?.ca
    }

    pub(crate) fn tls_key(&self) -> Option<String> {
        self.config?.tls?.key
    }

    pub(crate) fn tls_cert(&self) -> Option<String> {
        self.config?.tls?.cert
    }

    pub(crate) fn tls_key_unwrap(&self) -> String {
        self.config?.tls?.key.unwrap()
    }

    pub(crate) fn tls_cert_unwrap(&self) -> String {
        self.config?.tls?.cert.unwrap()
    }

    pub(crate) fn domain(&self) -> String {
        match self.config.unwrap().tls.unwrap().domain {
            Some(res) => res,
            None => "".to_string(),
        }
    }

    pub(crate) fn http_config(&self) -> Option<RequestCond> {
        match self.config.clone() {
            Some(res) => res.http,
            None => None,
        }
    }
}
