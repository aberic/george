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

use tokio::runtime::{Builder, Runtime};
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity, Uri};

use comm::errors::{Errs, GeorgeResult};

use crate::protos::utils::utils::Status;
use crate::tools::Trans;

pub mod db;

fn endpoint(
    remote: &str,
    port: u16,
    tls: bool,
    key: Option<Vec<u8>>,
    cert: Option<Vec<u8>>,
    server_ca: Option<Vec<u8>>,
    domain_name: impl Into<String>,
) -> GeorgeResult<(Channel, Runtime)> {
    let dst = format!("{}://{}:{}", "http", remote, port);
    let rt: Runtime;
    match Builder::new_multi_thread().enable_all().build() {
        Ok(res) => rt = res,
        Err(err) => return Err(Errs::strs("failed to obtain a new RunTime object!", err)),
    }
    let uri;
    match Uri::from_maybe_shared(dst) {
        Ok(res) => uri = res,
        Err(err) => return Err(Errs::strs("uri from maybe shared", err)),
    }
    let mut endpoint = Channel::builder(uri);
    if tls {
        let mut tls_config = ClientTlsConfig::new().domain_name(domain_name);
        match key {
            Some(res) => {
                let key = res;
                match cert {
                    Some(res) => {
                        let cert = res;
                        let identity = Identity::from_pem(cert, key);
                        tls_config = tls_config.identity(identity);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        match server_ca {
            Some(res) => {
                let server_ca = res;
                let cert = Certificate::from_pem(server_ca);
                tls_config = tls_config.ca_certificate(cert);
            }
            _ => {}
        }
        endpoint = endpoint.tls_config(tls_config).unwrap();
    }
    // endpoint = endpoint.timeout(Duration::from_secs(30));
    let future = endpoint.connect();
    match rt.block_on(future) {
        Ok(res) => Ok((res, rt)),
        Err(err) => Err(Errs::strs("endpoint connect", err)),
    }
}

fn status_check(status_i32: i32, msg_err: String) -> GeorgeResult<()> {
    let status = Trans::i32_2_status(status_i32)?;
    match status {
        Status::Ok => Ok(()),
        _ => Err(Errs::string(format!(
            "failed to create! status is {}, error is {}",
            status_i32, msg_err
        ))),
    }
}
