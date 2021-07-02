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

use crate::client::db::DatabaseRpcClient;
use crate::protos::db::db::database_service_client::DatabaseServiceClient;
use crate::protos::db::db::ResponseDatabaseList;
use crate::protos::utils::utils::Req;

impl DatabaseRpcClient {
    pub fn new(
        remote: &str,
        port: u16,
        tls: bool,
        key: Option<Vec<u8>>,
        cert: Option<Vec<u8>>,
        server_ca: Option<Vec<u8>>,
    ) -> GeorgeResult<DatabaseRpcClient> {
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
            let mut tls_config = ClientTlsConfig::new().domain_name("example.com");
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
            Ok(res) => Ok(DatabaseRpcClient {
                client: DatabaseServiceClient::new(res),
                rt,
            }),
            Err(err) => Err(Errs::strs("endpoint connect", err)),
        }
    }
}

impl DatabaseRpcClient {
    pub fn list(&mut self) -> GeorgeResult<ResponseDatabaseList> {
        let request = tonic::Request::new(Req {});
        match self.rt.block_on(self.client.list(request)) {
            Ok(res) => Ok(res.into_inner()),
            Err(err) => Err(Errs::strs(
                "failed to successfully run the future on RunTime!",
                err,
            )),
        }
    }
}
