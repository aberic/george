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
    use comm::io::file::FilerReader;
    use comm::io::Filer;

    use crate::client::db::DatabaseRpcClient;

    #[test]
    fn list_tls_cross() {
        let server_ca = Some(Filer::read_bytes("src/examples/ca.pem").unwrap());
        let mut cli = DatabaseRpcClient::new(
            "127.0.0.1",
            9219,
            true,
            None,
            None,
            server_ca,
            "example.com",
        )
        .unwrap();
        let res = cli.list().unwrap();
        for db in res {
            println!("db {}", db.name)
        }
    }

    #[test]
    fn list_tls_terraform() {
        let server_ca = Some(Filer::read_bytes("src/examples/terraform/ca.pem").unwrap());
        let mut cli = DatabaseRpcClient::new(
            "127.0.0.1",
            9219,
            true,
            None,
            None,
            server_ca,
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
        let server_ca = Some(Filer::read_bytes("src/examples/terraform/ca.pem").unwrap());
        let mut cli = DatabaseRpcClient::new(
            "127.0.0.1",
            9219,
            true,
            None,
            None,
            server_ca,
            "foo.test.google.fr",
        )
        .unwrap();
        let res = cli.list().unwrap();
        for db in res {
            println!("db {}", db.name)
        }
    }

    #[test]
    fn list_tls_1() {
        let server_ca = Some(Filer::read_bytes("src/examples/tls/server_ca.pem").unwrap());
        let mut cli = DatabaseRpcClient::new(
            "127.0.0.1",
            9219,
            true,
            None,
            None,
            server_ca,
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
        let server_ca = Some(Filer::read_bytes("src/examples/pki/rsa/client.fullchain").unwrap());
        let mut cli = DatabaseRpcClient::new(
            "127.0.0.1",
            9219,
            true,
            None,
            None,
            server_ca,
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
        let key = Some(Filer::read_bytes("src/examples/tls/client_sk.key").unwrap());
        let cert = Some(Filer::read_bytes("src/examples/tls/client.pem").unwrap());
        let server_ca = Some(Filer::read_bytes("src/examples/tls/server_ca.pem").unwrap());
        let mut cli =
            DatabaseRpcClient::new("127.0.0.1", 9219, true, key, cert, server_ca, "example.com")
                .unwrap();
        let res = cli.list().unwrap();
        for db in res {
            println!("db {}", db.name)
        }
    }
}

#[cfg(test)]
mod database {
    use crate::client::db::DatabaseRpcClient;
    use crate::tools::Trans;

    #[test]
    fn list() {
        let mut cli =
            DatabaseRpcClient::new("127.0.0.1", 9219, false, None, None, None, "").unwrap();
        let res = cli.list().unwrap();
        for db in res {
            println!("db {}", db.name)
        }
    }

    #[test]
    fn create() {
        let mut cli =
            DatabaseRpcClient::new("127.0.0.1", 9219, false, None, None, None, "").unwrap();
        cli.create("test".to_string(), "test comment".to_string())
            .unwrap();
        let res = cli.list().unwrap();
        for db in res {
            println!("db {}", db.name)
        }
    }

    #[test]
    fn info() {
        let mut cli =
            DatabaseRpcClient::new("127.0.0.1", 9219, false, None, None, None, "").unwrap();
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
