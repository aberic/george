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

use std::option::Option::{None, Some};
use std::path::Path;
use std::time::Duration;

use tokio::runtime::{Builder, Runtime};
use tonic::transport::{Channel, Endpoint, Uri};

use george_comm::errors::{Errs, GeorgeResult};
use george_comm::io::file::FilerReader;
use george_comm::io::Filer;

use crate::protos::utils::utils::Status;
use crate::tools::Trans;

pub mod db;
mod notls;
mod openssl;
mod rustls;

/// `TLS`请求新建公共方法
pub trait TLS {
    fn new<P: AsRef<Path>>(
        remote: &str,
        port: u16,
        ca_path: P,
        domain_name: impl Into<String>,
        cond_op: Option<RequestCond>,
    ) -> GeorgeResult<(Channel, Runtime)>;

    fn new_check<P: AsRef<Path>>(
        remote: &str,
        port: u16,
        key_path: P,
        cert_path: P,
        ca_path: P,
        domain_name: impl Into<String>,
        cond_op: Option<RequestCond>,
    ) -> GeorgeResult<(Channel, Runtime)>;

    fn new_bytes(
        remote: &str,
        port: u16,
        ca_bytes: Vec<u8>,
        domain_name: impl Into<String>,
        cond_op: Option<RequestCond>,
    ) -> GeorgeResult<(Channel, Runtime)>;

    fn new_bytes_check(
        remote: &str,
        port: u16,
        key_bytes: Vec<u8>,
        cert_bytes: Vec<u8>,
        ca_bytes: Vec<u8>,
        domain_name: impl Into<String>,
        cond_op: Option<RequestCond>,
    ) -> GeorgeResult<(Channel, Runtime)>;
}

/// `RPC`客户端请求新建公共方法
pub trait RpcClient {
    fn new(remote: &str, port: u16, cond_op: Option<RequestCond>) -> GeorgeResult<Self>
    where
        Self: Sized;

    fn new_tls<P: AsRef<Path>>(
        tls_type: TLSType,
        remote: &str,
        port: u16,
        ca_path: P,
        domain_name: impl Into<String>,
        cond_op: Option<RequestCond>,
    ) -> GeorgeResult<Self>
    where
        Self: Sized,
    {
        let ca_bytes = Filer::read_bytes(ca_path)?;
        RpcClient::new_tls_bytes(tls_type, remote, port, ca_bytes, domain_name, cond_op)
    }

    fn new_tls_bytes(
        tls_type: TLSType,
        remote: &str,
        port: u16,
        ca_bytes: Vec<u8>,
        domain_name: impl Into<String>,
        cond_op: Option<RequestCond>,
    ) -> GeorgeResult<Self>
    where
        Self: Sized;

    fn new_tls_check<P: AsRef<Path>>(
        tls_type: TLSType,
        remote: &str,
        port: u16,
        key_path: P,
        cert_path: P,
        ca_path: P,
        domain_name: impl Into<String>,
        cond_op: Option<RequestCond>,
    ) -> GeorgeResult<Self>
    where
        Self: Sized,
    {
        let key_bytes = Filer::read_bytes(ca_path)?;
        let cert_bytes = Filer::read_bytes(key_path)?;
        let ca_bytes = Filer::read_bytes(cert_path)?;
        RpcClient::new_tls_bytes_check(
            tls_type,
            remote,
            port,
            key_bytes,
            cert_bytes,
            ca_bytes,
            domain_name,
            cond_op,
        )
    }

    fn new_tls_bytes_check(
        tls_type: TLSType,
        remote: &str,
        port: u16,
        key_bytes: Vec<u8>,
        cert_bytes: Vec<u8>,
        ca_bytes: Vec<u8>,
        domain_name: impl Into<String>,
        cond_op: Option<RequestCond>,
    ) -> GeorgeResult<Self>
    where
        Self: Sized;
}

struct Notls {
    uri: Uri,
}

struct Rustls {
    uri: Uri,
}

struct Openssl {
    uri: Uri,
}

/// TLS请求类型
#[derive(Debug, Clone, Copy)]
pub enum TLSType {
    Rustls,
    Openssl,
}

/// 请求条件设置结构
#[derive(Debug, Clone, Copy)]
pub struct RequestCond {
    /// 为所有请求处理程序设置超时，单位secs
    timeout: Option<u64>,
    /// 设置应用于每个连接入站请求的并发限制
    concurrency_limit: Option<usize>,
    /// 为接受的连接设置`TCP_NODELAY`选项的值。默认启用
    tcp_nodelay: Option<bool>,
    /// 设置是否在接受的连接上启用`TCP keepalive`消息，单位ms。<p>
    /// 如果指定了None，表示关闭keepalive功能，否则指定的持续时间就是发送TCP keepalive探测前的空闲时间。<p>
    /// 默认是没有keepalive (None)
    tcp_keepalive: Option<u64>,
    /// 设置是否在接受的连接上启用HTTP2 Ping帧，单位ms
    http2_keep_alive_interval: Option<u64>,
    /// 设置接收keepalive ping应答的超时时间，单位ms
    keep_alive_timeout: Option<u64>,
    /// 如果发送数据给对方，对方无响应，会等一段时间（Idle检测)，如果对方无响应，就会发送心跳包（Idle检测）
    keep_alive_while_idle: Option<bool>,
    /// 设置是否使用自适应流控制
    http2_adaptive_window: Option<bool>,
    /// 设置HTTP2的最大连接级流控制，默认是65535字节
    initial_connection_window_size: Option<u32>,
    /// 设置HTTP2流级别的流量控制的初始窗口大小，默认是65535字节
    initial_stream_window_size: Option<u32>,
    /// 对每个请求应用一个速率限制
    rate_limit: Option<RateLimit>,
}

/// 速率限制结构
/// 表示在`millis`时间段内，最多允许接收`limit`数量的报文
#[derive(Debug, Clone, Copy)]
pub struct RateLimit {
    /// 最多允许接收数量
    limit: u64,
    /// 限制时间段内
    millis: u64,
}

fn endpoint(mut endpoint: Endpoint, cond_op: Option<RequestCond>) -> GeorgeResult<Endpoint> {
    let cond;
    match cond_op {
        Some(res) => cond = res,
        None => return Ok(endpoint),
    }
    if let Some(res) = cond.timeout {
        endpoint = endpoint.timeout(Duration::from_secs(res));
    }
    if let Some(res) = cond.concurrency_limit {
        endpoint = endpoint.concurrency_limit(res);
    }
    if let Some(res) = cond.tcp_nodelay {
        endpoint = endpoint.tcp_nodelay(res);
    }
    if let Some(res) = cond.tcp_keepalive {
        endpoint = endpoint.tcp_keepalive(Some(Duration::from_millis(res)));
    }
    if let Some(res) = cond.http2_keep_alive_interval {
        endpoint = endpoint.http2_keep_alive_interval(Duration::from_millis(res));
    }
    if let Some(res) = cond.http2_adaptive_window {
        endpoint = endpoint.http2_adaptive_window(res);
    }
    if let Some(res) = cond.keep_alive_timeout {
        endpoint = endpoint.keep_alive_timeout(Duration::from_millis(res));
    }
    if let Some(res) = cond.keep_alive_while_idle {
        endpoint = endpoint.keep_alive_while_idle(res);
    }
    if let Some(res) = cond.initial_connection_window_size {
        endpoint = endpoint.initial_connection_window_size(res);
    }
    if let Some(res) = cond.initial_stream_window_size {
        endpoint = endpoint.initial_stream_window_size(res);
    }
    if let Some(res) = cond.rate_limit {
        endpoint = endpoint.rate_limit(res.limit, Duration::from_millis(res.millis));
    }
    Ok(endpoint)
}

fn runtime() -> GeorgeResult<Runtime> {
    match Builder::new_multi_thread().enable_all().build() {
        Ok(res) => Ok(res),
        Err(err) => return Err(Errs::strs("failed to obtain a new RunTime object!", err)),
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
