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

use serde::{Deserialize, Serialize};

use george_log::LogModule;

mod builder;
pub mod comm;
mod conf;
mod conf_test;
mod config;
mod db;
mod init;
mod log;
mod server;

pub struct Builder;

#[derive(Debug, PartialEq, Clone)]
pub struct Init {
    conf: Conf,
    log_main: LogModule,
}

/// yaml解析辅助结构
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Conf {
    config: Option<Config>,
}

/// 基础配置信息，优先读取环境变量中的结果<p>
///
/// 该配置信息可通过指定路径的文件中进行读取，文件格式支持yaml
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    log: Option<ConfigLog>,
    db: Option<ConfigDB>,
    server: Option<ConfigServer>,
}

/// 日志配置信息，优先读取环境变量中的结果<p>
///
/// 该配置信息可通过指定路径的文件中进行读取，文件格式支持yaml
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ConfigLog {
    /// 日志文件目录
    pub log_dir: Option<String>,
    /// 日志级别(debug/info/warn/Error/panic/fatal)
    pub log_level: Option<String>,
    /// 每个日志文件保存的最大尺寸 单位：M
    pub log_file_max_size: Option<u64>,
    /// 文件最多保存多少个
    pub log_file_max_count: Option<u32>,
    /// 是否在主日志文件中同步记录
    pub additive: Option<bool>,
}

/// 模块日志策略
pub struct LogPolicy {
    /// 日志文件目录
    dir: String,
    /// 日志文件名
    name: String,
    /// 日志截取包名，如：`george-db::task::master`
    pkg: String,
    /// 是否在主日志文件中同步记录
    additive: bool,
}

///  数据库配置信息，优先读取环境变量中的结果<p>
///
/// 该配置信息可通过指定路径的文件中进行读取，文件格式支持yaml
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ConfigDB {
    /// 服务数据存储路径
    pub data_dir: Option<String>,
    /// 限制打开文件描述符次数
    pub thread_count: Option<usize>,
}

/// 服务配置信息，优先读取环境变量中的结果<p>
///
/// 该配置信息可通过指定路径的文件中进行读取，文件格式支持yaml
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ConfigServer {
    /// 服务端口号
    pub port: Option<u16>,
    /// `TLS`配置
    pub tls: Option<ConfigServerTLS>,
    /// `HTTP`配置
    pub http: Option<ConfigServerHttp>,
}

/// 服务配置信息，优先读取环境变量中的结果<p>
///
/// 该配置信息可通过指定路径的文件中进行读取，文件格式支持yaml
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ConfigServerTLS {
    /// 服务端是否用`rustls`做校验，默认`openssl`，即`false`
    pub rust_tls: Option<bool>,
    /// 服务端`key`，开启`TLS`后生效
    pub key: Option<String>,
    /// 服务端`cert`，开启`TLS`后生效
    pub cert: Option<String>,
    /// 客户端根证书，开启`TLS`后生效
    pub ca: Option<String>,
    /// 服务端域名，开启`TLS`后生效
    pub domain: Option<String>,
}

/// 服务配置信息，优先读取环境变量中的结果<p>
///
/// 该配置信息可通过指定路径的文件中进行读取，文件格式支持yaml
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ConfigServerHttp {
    /// 为所有请求处理程序设置超时，单位secs
    pub timeout: Option<u64>,
    /// 设置应用于每个连接入站请求的并发限制
    pub concurrency_limit_per_connection: Option<usize>,
    /// 为接受的连接设置`TCP_NODELAY`选项的值。默认启用。
    ///
    /// 用户侧使用对于延时敏感型，同时数据传输量比较小的应用建议启用
    ///
    /// 服务侧相互同步数据，可依据实际情况选择禁用。
    /// 数据只有在写缓存中累积到一定量之后，才会被发送出去，这样明显提高了网络利用率（实际传输数据payload与协议头的比例大大提高）。
    /// 但会增加了延时；与TCP delayed ack这个特性结合，延时基本在40ms左右
    pub tcp_nodelay: Option<bool>,
    /// 设置是否在接受的连接上启用`TCP keepalive`消息，单位ms
    /// 如果指定了`None`，表示关闭`keepalive`功能，否则指定的持续时间就是发送`TCP keepalive`探测前的空闲时间。
    /// 默认是没有`keepalive` (None)
    pub tcp_keepalive: Option<u64>,
    /// 设置是否在接受的连接上启用HTTP2 Ping帧，单位ms<p>
    /// 如果指定了None，表示禁用HTTP2 keepalive，否则指定的持续时间为HTTP2 Ping帧之间的时间间隔。<p>
    /// 接收keepalive ping应答的超时时间可以设置为[Server::http2_keepalive_timeout]。<p>
    /// 默认是没有HTTP2 keepalive (None)
    ///
    /// Keep-alives一般被用来验证远端连接是否有效。
    /// 如果该连接上没有其他数据被传输，或者更高level 的 keep-alives被传送，keep-alives 在每个KeepAliveTime被发送。
    /// 如果没有收到 keep-alive 应答，keep-alive 将在每 KeepAliveInterval 秒重发一次。KeepAliveInterval 默认为5秒
    pub http2_keepalive_interval: Option<u64>,
    /// 设置接收keepalive ping应答的超时时间，单位ms<p>
    /// 如果在超时时间内没有确认ping，连接将被关闭。如果http2_keep_alive_interval被禁用，则不执行任何操作。<p>
    /// 默认值是20秒
    pub http2_keepalive_timeout: Option<u64>,
    /// 设置HTTP2的最大连接级流控制（以字节为单位），默认是65535
    pub initial_connection_window_size: Option<u32>,
    /// 表明发送方的流级别的流量控制的初始窗口大小（以字节为单位）。
    /// 初始值是2^16-1(65535)字节。
    /// 这个设置影响所有流的窗口大小。
    /// 超过65535的窗口大小必选被视为类型是FLOW_CONTROL_ERROR的连接错误
    pub initial_stream_window_size: Option<u32>,
    /// 限制对等端流的最大并发量，默认不限制
    pub max_concurrent_streams: Option<u32>,
    /// 设置HTTP2使用的最大帧大小。
    /// 如果未设置，将默认从底层传输。
    pub max_frame_size: Option<u32>,
}
