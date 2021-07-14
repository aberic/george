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

use tokio::runtime::Runtime;
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Endpoint, Identity, Uri};

use comm::errors::{Errs, GeorgeResult};
use comm::io::file::FilerReader;
use comm::io::Filer;

use crate::client::TLS;
use crate::client::{runtime, RequestCond, Rustls};

impl Rustls {
    fn new(remote: &str, port: u16) -> GeorgeResult<Rustls> {
        let dst = format!("{}://{}:{}", "http", remote, port);
        match Uri::from_maybe_shared(dst) {
            Ok(res) => Ok(Rustls { uri: res }),
            Err(err) => Err(Errs::strs("uri from maybe shared", err)),
        }
    }

    fn uri(&self) -> Uri {
        self.uri.clone()
    }

    fn block_on(
        &self,
        mut endpoint: Endpoint,
        cond_op: Option<RequestCond>,
    ) -> GeorgeResult<(Channel, Runtime)> {
        println!("rustls");
        let rt = runtime()?;
        endpoint = crate::client::endpoint(endpoint, cond_op)?;
        let future = endpoint.connect();
        match rt.block_on(future) {
            Ok(res) => Ok((res, rt)),
            Err(err) => Err(Errs::strs("endpoint connect", err)),
        }
    }
}

impl TLS for Rustls {
    fn new<P: AsRef<Path>>(
        remote: &str,
        port: u16,
        ca_path: P,
        domain_name: impl Into<String>,
        cond_op: Option<RequestCond>,
    ) -> GeorgeResult<(Channel, Runtime)> {
        let ca_bytes = Filer::read_bytes(ca_path)?;
        Rustls::new_bytes(remote, port, ca_bytes, domain_name, cond_op)
    }

    fn new_check<P: AsRef<Path>>(
        remote: &str,
        port: u16,
        key_path: P,
        cert_path: P,
        ca_path: P,
        domain_name: impl Into<String>,
        cond_op: Option<RequestCond>,
    ) -> GeorgeResult<(Channel, Runtime)> {
        let key_bytes = Filer::read_bytes(key_path)?;
        let cert_bytes = Filer::read_bytes(cert_path)?;
        let ca_bytes = Filer::read_bytes(ca_path)?;
        Rustls::new_bytes_check(
            remote,
            port,
            key_bytes,
            cert_bytes,
            ca_bytes,
            domain_name,
            cond_op,
        )
    }

    fn new_bytes(
        remote: &str,
        port: u16,
        ca_bytes: Vec<u8>,
        domain_name: impl Into<String>,
        cond_op: Option<RequestCond>,
    ) -> GeorgeResult<(Channel, Runtime)> {
        let tls = Rustls::new(remote, port)?;
        let mut endpoint = Channel::builder(tls.uri());

        let mut tls_config = ClientTlsConfig::new().domain_name(domain_name);
        let cert = Certificate::from_pem(ca_bytes);
        tls_config = tls_config.ca_certificate(cert);

        endpoint = endpoint.tls_config(tls_config).unwrap();
        tls.block_on(endpoint, cond_op)
    }

    fn new_bytes_check(
        remote: &str,
        port: u16,
        key_bytes: Vec<u8>,
        cert_bytes: Vec<u8>,
        ca_bytes: Vec<u8>,
        domain_name: impl Into<String>,
        cond_op: Option<RequestCond>,
    ) -> GeorgeResult<(Channel, Runtime)> {
        let tls = Rustls::new(remote, port)?;
        let mut endpoint = Channel::builder(tls.uri());

        let mut tls_config = ClientTlsConfig::new().domain_name(domain_name);
        let identity = Identity::from_pem(cert_bytes, key_bytes);
        tls_config = tls_config.identity(identity);
        let cert = Certificate::from_pem(ca_bytes);
        tls_config = tls_config.ca_certificate(cert);

        endpoint = endpoint.tls_config(tls_config).unwrap();
        tls.block_on(endpoint, cond_op)
    }
}
