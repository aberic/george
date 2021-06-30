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

use crate::ConfigServer;

impl ConfigServer {
    pub fn default() -> ConfigServer {
        ConfigServer {
            tls: None,
            tls_key: None,
            tls_cert: None,
            tls_client_root_cert: None,
            timeout: None,
            concurrency_limit_per_connection: None,
            tcp_nodelay: None,
            tcp_keepalive: None,
            http2_keepalive_interval: None,
            http2_keepalive_timeout: None,
            initial_connection_window_size: None,
            initial_stream_window_size: None,
            max_concurrent_streams: None,
            max_frame_size: None,
            port: Some(9219),
        }
    }

    pub(crate) fn check(&mut self) {
        match self.port {
            None => self.port = Some(9219),
            _ => {}
        }
    }
}
