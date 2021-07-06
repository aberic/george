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
    use crate::client::db::DatabaseRpcClient;
    use comm::io::file::FilerReader;
    use comm::io::Filer;

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

    #[test]
    fn list() {
        let mut cli =
            DatabaseRpcClient::new("127.0.0.1", 9219, false, None, None, None, "").unwrap();
        let res = cli.list().unwrap();
        for db in res {
            println!("db {}", db.name)
        }
    }
}
