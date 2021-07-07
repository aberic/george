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
mod database_tls {

    use crate::client::db::{DatabaseRpcClient, RpcClient};

    #[test]
    fn list_tls_cross() {
        let mut cli =
            DatabaseRpcClient::new_tls("127.0.0.1", 9219, "src/examples/ca.pem", "example.com")
                .unwrap();
        let res = cli.list().unwrap();
        for db in res {
            println!("db {}", db.name)
        }
    }

    #[test]
    fn list_tls_terraform() {
        let mut cli = DatabaseRpcClient::new_tls(
            "127.0.0.1",
            9219,
            "src/examples/terraform/ca.pem",
            "foo.test.google.fr".to_string(),
        )
        .unwrap();
        let res = cli.list().unwrap();
        for db in res {
            println!("db {}", db.name)
        }
    }

    #[test]
    fn list_tls_terraform_str() {
        let mut cli = DatabaseRpcClient::new_tls(
            "127.0.0.1",
            9219,
            "src/examples/terraform/ca.pem",
            "foo.test.google.fr",
        )
        .unwrap();
        let res = cli.list().unwrap();
        for db in res {
            println!("db {}", db.name)
        }
    }

    #[test]
    fn list_tls_terraform_1() {
        let mut cli = DatabaseRpcClient::new_tls_check(
            "127.0.0.1",
            9219,
            "src/examples/terraform/server1.key",
            "src/examples/terraform/server1.pem",
            "src/examples/terraform/ca.pem",
            "foo.test.google.fr".to_string(),
        )
        .unwrap();
        let res = cli.list().unwrap();
        for db in res {
            println!("db {}", db.name)
        }
    }

    #[test]
    fn list_tls_1() {
        let mut cli = DatabaseRpcClient::new_tls(
            "127.0.0.1",
            9219,
            "src/examples/tls/server_ca.pem",
            "example.com",
        )
        .unwrap();
        let res = cli.list().unwrap();
        for db in res {
            println!("db {}", db.name)
        }
    }

    #[test]
    fn list_pki_1() {
        let mut cli = DatabaseRpcClient::new_tls(
            "127.0.0.1",
            9219,
            "src/examples/pki/rsa/client.fullchain",
            "example.com",
        )
        .unwrap();
        let res = cli.list().unwrap();
        for db in res {
            println!("db {}", db.name)
        }
    }

    #[test]
    fn list_tls() {
        let mut cli = DatabaseRpcClient::new_tls_check(
            "127.0.0.1",
            9219,
            "src/examples/tls/client_sk.key",
            "src/examples/tls/client.pem",
            "src/examples/tls/server_ca.pem",
            "example.com",
        )
        .unwrap();
        let res = cli.list().unwrap();
        for db in res {
            println!("db {}", db.name)
        }
    }
}

#[cfg(test)]
mod database {
    use crate::client::db::{DatabaseRpcClient, RpcClient};
    use crate::tools::Trans;

    #[test]
    fn list() {
        let mut cli = DatabaseRpcClient::new("127.0.0.1", 9219).unwrap();
        let res = cli.list().unwrap();
        for db in res {
            println!("db {}", db.name)
        }
    }

    #[test]
    fn create() {
        let mut cli = DatabaseRpcClient::new("127.0.0.1", 9219).unwrap();
        cli.create("test".to_string(), "test comment".to_string())
            .unwrap();
        let res = cli.list().unwrap();
        for db in res {
            println!("db {}", db.name)
        }
    }

    #[test]
    fn info() {
        let mut cli = DatabaseRpcClient::new("127.0.0.1", 9219).unwrap();
        let res = cli.info("test".to_string()).unwrap();
        println!(
            "db name = {}, comment = {}, create_time = {}",
            res.name,
            res.comment,
            Trans::grpc_timestamp_2_string(res.create_time.unwrap().seconds)
        );
        for view in res.views {
            println!(
                "view name = {}, comment = {}, version = {}, filepath = {}, create_time = {}",
                view.name,
                view.comment,
                view.version,
                view.filepath,
                Trans::grpc_timestamp_2_string(view.create_time.unwrap().seconds)
            );
            for index in view.indexes {
                println!("index name = {}, engine = {}, key_type = {}, primary = {}, unique = {}, null = {}, create_time = {}", index.name, index.engine, index.key_type, index.primary, index.unique, index.null, Trans::grpc_timestamp_2_string(index.create_time.unwrap().seconds));
            }
        }
    }
}
