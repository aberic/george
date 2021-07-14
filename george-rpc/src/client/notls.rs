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

use tokio::runtime::Runtime;
use tonic::transport::{Channel, Endpoint, Uri};

use george_comm::errors::{Errs, GeorgeResult};

use crate::client::{runtime, Notls, RequestCond};

impl Notls {
    fn new(remote: &str, port: u16) -> GeorgeResult<Notls> {
        let dst = format!("{}://{}:{}", "http", remote, port);
        match Uri::from_maybe_shared(dst) {
            Ok(res) => Ok(Notls { uri: res }),
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
        println!("notls");
        let rt = runtime()?;
        endpoint = crate::client::endpoint(endpoint, cond_op)?;
        let future = endpoint.connect();
        match rt.block_on(future) {
            Ok(res) => Ok((res, rt)),
            Err(err) => Err(Errs::strs("endpoint connect", err)),
        }
    }

    pub(crate) fn make(
        remote: &str,
        port: u16,
        cond_op: Option<RequestCond>,
    ) -> GeorgeResult<(Channel, Runtime)> {
        let tls = Notls::new(remote, port)?;
        let endpoint = Channel::builder(tls.uri());
        tls.block_on(endpoint, cond_op)
    }
}
