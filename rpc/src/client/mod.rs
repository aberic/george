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
use tokio::runtime::{Builder, Runtime};
// use tonic::codegen::*;
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity, Uri};

use comm::errors::{Errs, GeorgeResult};
use comm::io::file::FilerReader;
use comm::io::Filer;

use crate::protos::utils::utils::Status;
use crate::tools::Trans;
// use comm::cryptos::Cert;
// use comm::openssl::tonic::ALPN_H2_WIRE;
// use hyper::client::HttpConnector;
// use hyper::client::{Client, ResponseFuture};
// use hyper::Request;
// use hyper_openssl::HttpsConnector;
// use openssl::ssl::{SslConnector, SslMethod};
// use tonic::body::BoxBody;
// use tower::util::ServiceFn;

pub mod db;

fn request_resource(remote: &str, port: u16, openssl: bool) -> GeorgeResult<(Runtime, Uri)> {
    let dst: String;
    if openssl {
        dst = format!("{}://{}:{}", "https", remote, port);
    } else {
        dst = format!("{}://{}:{}", "http", remote, port);
    }
    let rt: Runtime;
    match Builder::new_multi_thread().enable_all().build() {
        Ok(res) => rt = res,
        Err(err) => return Err(Errs::strs("failed to obtain a new RunTime object!", err)),
    }
    match Uri::from_maybe_shared(dst) {
        Ok(res) => Ok((rt, res)),
        Err(err) => Err(Errs::strs("uri from maybe shared", err)),
    }
}

fn endpoint(remote: &str, port: u16) -> GeorgeResult<(Channel, Runtime)> {
    let (rt, uri) = request_resource(remote, port, false)?;
    let endpoint = Channel::builder(uri);
    // endpoint = endpoint.timeout(Duration::from_secs(30));
    let future = endpoint.connect();
    match rt.block_on(future) {
        Ok(res) => Ok((res, rt)),
        Err(err) => Err(Errs::strs("endpoint connect", err)),
    }
}

fn endpoint_tls<P: AsRef<Path>>(
    remote: &str,
    port: u16,
    ca_path: P,
    domain_name: impl Into<String>,
) -> GeorgeResult<(Channel, Runtime)> {
    let ca = Filer::read_bytes(ca_path)?;
    endpoint_tls_bytes(remote, port, ca, domain_name)

    // let (rt, uri) = request_resource(remote, port, true)?;
    // let mut ssl = SslConnector::builder(SslMethod::tls_client()).unwrap();
    // // let ca_bytes = Filer::read_bytes(ca_path)?;
    // // let cert = Cert::load_pem(ca_bytes)?.x509;
    // // connector_builder.cert_store_mut().add_cert(cert);
    // ssl.set_ca_file(ca_path).unwrap();
    // ssl.set_alpn_protos(ALPN_H2_WIRE).unwrap();
    //
    // let mut http = HttpConnector::new();
    // http.enforce_http(false);
    // let mut https = HttpsConnector::with_connector(http, ssl).unwrap();
    //
    // https.set_callback(|c, _| {
    //     c.set_verify_hostname(false);
    //     Ok(())
    // });
    //
    // let hyper = Client::builder().http2_only(true).build(https);
    //
    // let add_origin = tower::service_fn(|mut req: hyper::Request<tonic::body::BoxBody>| {
    //     let uri = Uri::builder()
    //         .scheme(uri.scheme().unwrap().clone())
    //         .authority(uri.authority().unwrap().clone())
    //         .path_and_query(req.uri().path_and_query().unwrap().clone())
    //         .build()
    //         .unwrap();
    //
    //     *req.uri_mut() = uri;
    //
    //     hyper.request(req)
    // });
    //
    // Ok((add_origin, rt))
}

fn endpoint_tls_check<P: AsRef<Path>>(
    remote: &str,
    port: u16,
    key_path: P,
    cert_path: P,
    ca_path: P,
    domain_name: impl Into<String>,
) -> GeorgeResult<(Channel, Runtime)> {
    let key = Filer::read_bytes(key_path)?;
    let cert = Filer::read_bytes(cert_path)?;
    let ca = Filer::read_bytes(ca_path)?;
    endpoint_tls_check_bytes(remote, port, key, cert, ca, domain_name)
}

fn endpoint_tls_bytes(
    remote: &str,
    port: u16,
    ca: Vec<u8>,
    domain_name: impl Into<String>,
) -> GeorgeResult<(Channel, Runtime)> {
    let (rt, uri) = request_resource(remote, port, false)?;
    let mut endpoint = Channel::builder(uri);

    let mut tls_config = ClientTlsConfig::new().domain_name(domain_name);
    let cert = Certificate::from_pem(ca);
    tls_config = tls_config.ca_certificate(cert);

    endpoint = endpoint.tls_config(tls_config).unwrap();
    // endpoint = endpoint.timeout(Duration::from_secs(30));
    let future = endpoint.connect();
    match rt.block_on(future) {
        Ok(res) => Ok((res, rt)),
        Err(err) => Err(Errs::strs("endpoint connect", err)),
    }
}

fn endpoint_tls_check_bytes(
    remote: &str,
    port: u16,
    key: Vec<u8>,
    cert: Vec<u8>,
    ca: Vec<u8>,
    domain_name: impl Into<String>,
) -> GeorgeResult<(Channel, Runtime)> {
    let (rt, uri) = request_resource(remote, port, false)?;
    let mut endpoint = Channel::builder(uri);

    let mut tls_config = ClientTlsConfig::new().domain_name(domain_name);
    let identity = Identity::from_pem(cert, key);
    tls_config = tls_config.identity(identity);
    let cert = Certificate::from_pem(ca);
    tls_config = tls_config.ca_certificate(cert);

    endpoint = endpoint.tls_config(tls_config).unwrap();
    // let connector = SslConnector::builder(SslMethod::tls_client()).unwrap();
    // endpoint = endpoint.timeout(Duration::from_secs(30));
    // let future = endpoint.connect_with_connector(connector);
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
            "failed! status is {}, error is {}",
            status_i32, msg_err
        ))),
    }
}
