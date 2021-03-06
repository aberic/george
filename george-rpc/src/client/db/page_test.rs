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

#[cfg(test)]
mod page {
    use crate::client::db::PageRpcClient;
    use crate::client::RpcClient;
    use crate::tools::Trans;

    #[test]
    fn list() {
        let mut cli = PageRpcClient::new("127.0.0.1", 9219, None).unwrap();
        let res = cli.list().unwrap();
        for page in res {
            println!("page {}", page.name)
        }
    }

    #[test]
    fn create() {
        let mut cli = PageRpcClient::new("127.0.0.1", 9219, None).unwrap();
        cli.create("test".to_string(), "test comment".to_string(), 0, 300)
            .unwrap();
        let res = cli.list().unwrap();
        for page in res {
            println!("page {}", page.name)
        }
    }

    #[test]
    fn info() {
        let mut cli = PageRpcClient::new("127.0.0.1", 9219, None).unwrap();
        let res = cli.info("test".to_string()).unwrap();
        println!(
            "page name = {}, comment = {}, comment = {}, size = {}, period = {}",
            res.name,
            res.comment,
            res.size,
            res.period,
            Trans::grpc_timestamp_2_string(res.create_time.unwrap().seconds)
        );
    }
}
