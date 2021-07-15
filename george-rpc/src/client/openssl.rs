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

use hyper::client::HttpConnector;
use hyper_openssl::HttpsConnector;
use openssl::ssl::{SslConnector, SslMethod};
use tokio::runtime::Runtime;
use tonic::transport::{Channel, Endpoint, Uri};

use george_comm::cryptos::{Cert, Key};
use george_comm::errors::{Errs, GeorgeResult};
use george_comm::io::file::FilerReader;
use george_comm::io::Filer;
use george_comm::openssl::tonic::ALPN_H2_WIRE;

use crate::client::TLS;
use crate::client::{runtime, Openssl, RequestCond};

impl Openssl {
    fn new(remote: &str, port: u16) -> GeorgeResult<Openssl> {
        let dst = format!("{}://{}:{}", "https", remote, port);
        match Uri::from_maybe_shared(dst) {
            Ok(res) => Ok(Openssl { uri: res }),
            Err(err) => Err(Errs::strs("uri from maybe shared", err)),
        }
    }

    fn uri(&self) -> Uri {
        self.uri.clone()
    }

    fn block_on(
        &self,
        mut endpoint: Endpoint,
        https: HttpsConnector<HttpConnector>,
        cond_op: Option<RequestCond>,
    ) -> GeorgeResult<(Channel, Runtime)> {
        println!("openssl");
        let rt = runtime()?;
        endpoint = crate::client::endpoint(endpoint, cond_op)?;
        let future = endpoint.connect_with_connector(https);
        match rt.block_on(future) {
            Ok(res) => Ok((res, rt)),
            Err(err) => Err(Errs::strs("endpoint connect", err)),
        }
    }
}

impl TLS for Openssl {
    fn new<P: AsRef<Path>>(
        remote: &str,
        port: u16,
        ca_path: P,
        domain_name: impl Into<String>,
        cond_op: Option<RequestCond>,
    ) -> GeorgeResult<(Channel, Runtime)> {
        let ca_bytes = Filer::read_bytes(ca_path)?;
        Openssl::new_bytes(remote, port, ca_bytes, domain_name, cond_op)
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
        Openssl::new_bytes_check(
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
        let tls = Openssl::new(remote, port)?;
        let mut ssl = SslConnector::builder(SslMethod::tls_client()).unwrap();
        let ca = Cert::load_pem(ca_bytes)?.x509;
        ssl.cert_store_mut().add_cert(ca).unwrap();
        // ssl.set_ca_file(ca_path).unwrap();
        ssl.set_alpn_protos(ALPN_H2_WIRE).unwrap();

        let mut http = HttpConnector::new();
        http.enforce_http(false);
        let mut https = HttpsConnector::with_connector(http, ssl).unwrap();

        let domain_name = domain_name.into();
        https.set_callback(move |c, _| {
            if domain_name.is_empty() {
                c.set_verify_hostname(false);
            } else {
                c.set_hostname(domain_name.as_str()).unwrap()
            }
            Ok(())
        });

        let endpoint = Channel::builder(tls.uri());
        tls.block_on(endpoint, https, cond_op)
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
        let tls = Openssl::new(remote, port)?;

        let ca = Cert::load_pem(ca_bytes)?.x509;
        let sk = Key::load_sk_bytes(key_bytes)?;
        let cert = Cert::load_pem(cert_bytes)?.x509;

        let mut ssl = SslConnector::builder(SslMethod::tls_client()).unwrap();
        ssl.cert_store_mut().add_cert(ca).unwrap();
        // ssl.set_ca_file(ca_path).unwrap();
        ssl.set_private_key(sk.as_ref()).unwrap();
        ssl.set_certificate(cert.as_ref()).unwrap();
        ssl.set_alpn_protos(ALPN_H2_WIRE).unwrap();

        let mut http = HttpConnector::new();
        http.enforce_http(false);
        let mut https = HttpsConnector::with_connector(http, ssl).unwrap();

        let domain_name = domain_name.into();
        https.set_callback(move |c, _| {
            if domain_name.is_empty() {
                c.set_verify_hostname(false);
            } else {
                c.set_hostname(domain_name.as_str()).unwrap()
            }
            Ok(())
        });

        let endpoint = Channel::builder(tls.uri());
        tls.block_on(endpoint, https, cond_op)
    }
}
