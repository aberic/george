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

#[cfg(test)]
mod yaml {
    use serde::{Deserialize, Serialize};

    use crate::yaml::{Exec, Get, Handler, New};
    use crate::{Yaml, YamlArray};

    #[derive(Debug, Serialize, Deserialize)]
    struct User {
        name: String,
        age: u8,
        blog: String,
        addr: String,
    }

    const DATA: &str = r#"
                            name: John Doe
                            age: 43
                            phones:
                                - +44 1234567
                                - +44 2345678
                        "#;
    const USER: &str = r#"
                            name: 琼台博客
                            age: 30
                            blog: "https://www.qttc.net"
                            addr: 4114 Sepulveda Blvd
                        "#;
    const GET: &str = r#"
                            string: text
                            u64: 127
                            i64: -128
                            f64: 549.127
                            bool: false
                            object:
                              string: text
                              u64: 127
                              i64: -128
                              f64: 549.127
                              bool: false
                        "#;
    const ARRAY: &str = r#"
                            string: text
                            u64: 127
                            i64: -128
                            f64: 549.127
                            bool: false
                            object:
                              string: text
                              u64: 127
                              i64: -128
                              f64: 549.127
                              bool: false
                            array1:
                                - string: text
                                  u64: 127
                                  i64: -128
                                  f64: 549.127
                                  bool: false
                                  array: 
                                    - hello
                                    - world
                                    - test
                                - string: text
                                  u64: 127
                                  i64: -128
                                  f64: 549.127
                                  bool: false
                                  array: 
                                    - 1
                                    - 100
                                    - 10000
                                - string: text
                                  u64: 127
                                  i64: -128
                                  f64: 549.127
                                  bool: false
                                  array: 
                                    - 5.4
                                    - 100.1
                                    - 10000.98
                            array2: 
                                - one
                                - two
                                - three: object
                        "#;
    const ARRAYS: &str = r#"
                            - string: text
                              u64: 127
                              i64: -128
                              f64: 549.127
                              bool: false
                              array: 
                                - hello
                                - world
                                - test
                            - string: text
                              u64: 127
                              i64: -128
                              f64: 549.127
                              bool: false
                              array: 
                                - 1
                                - 100
                                - 10000
                            - string: text
                              u64: 127
                              i64: -128
                              f64: 549.127
                              bool: false
                              array: 
                                - 5.4
                                - 100.1
                                - 10000.98
                            - string: text
                              u64: 127
                              i64: -128
                              f64: 549.127
                              bool: false
                              array: 
                                - 5.4
                                - test
                                - 10000
                                - false
                                - -99
                          "#;
    const ARRAY_OBJECT: &str = r#"
                                - name: 琼台博客
                                  age: 30
                                  blog: "https://www.qttc.net"
                                  addr: 4114 Sepulveda Blvd
                                - name: 琼台博客
                                  age: 30
                                  blog: "https://www.qttc.net"
                                  addr: 4114 Sepulveda Blvd
                               "#;
    const OBJECT: &str = r#"
                            version: '3.4'

                            services:
                            
                              zookeeper1:
                                image: hyperledger/fabric-zookeeper:0.4.10
                                ports:
                                  - "2181:2181"
                                  - "2881:2888"
                                  - "3881:3888"
                                environment:
                                  - ZOO_MY_ID=1
                                  - ZOO_SERVERS=server.1=0.0.0.0:2881:3881 server.2=fabric_zookeeper2:2882:3882 server.3=fabric_zookeeper3:2883:3883
                                volumes:
                                    - /mount/fabric/test/back/zk1:/data
                                networks:
                                  - test
                                deploy:
                                  mode: replicated
                                  replicas: 1
                                  restart_policy:
                                    condition: on-failure
                                    delay: 5s
                                    max_attempts: 3
                                  update_config:
                                    parallelism: 1
                                    delay: 10s
                            
                              zookeeper2:
                                image: hyperledger/fabric-zookeeper:0.4.10
                                ports:
                                  - "2182:2181"
                                  - "2882:2888"
                                  - "3882:3888"
                                environment:
                                  - ZOO_MY_ID=2
                                  - ZOO_SERVERS=server.1=fabric_zookeeper1:2881:3881 server.2=0.0.0.0:2882:3882 server.3=fabric_zookeeper3:2883:3883
                                volumes:
                                    - /mount/fabric/test/back/zk2:/data
                                networks:
                                  - test
                                deploy:
                                  mode: replicated
                                  replicas: 1
                                  restart_policy:
                                    condition: on-failure
                                    delay: 5s
                                    max_attempts: 3
                                  update_config:
                                    parallelism: 1
                                    delay: 10s
                            
                              zookeeper3:
                                image: hyperledger/fabric-zookeeper:0.4.10
                                ports:
                                  - "2183:2181"
                                  - "2883:2888"
                                  - "3883:3888"
                                environment:
                                  - ZOO_MY_ID=3
                                  - ZOO_SERVERS=server.1=fabric_zookeeper1:2881:3881 server.2=fabric_zookeeper2:2882:3882 server.3=0.0.0.0:2883:3883
                                volumes:
                                    - /mount/fabric/test/back/zk3:/data
                                networks:
                                  - test
                                deploy:
                                  mode: replicated
                                  replicas: 1
                                  restart_policy:
                                    condition: on-failure
                                    delay: 5s
                                    max_attempts: 3
                                  update_config:
                                    parallelism: 1
                                    delay: 10s
                            
                              kafka1:
                                image: hyperledger/fabric-kafka:0.4.10
                                ports:
                                  - "9091:9092"
                                environment:
                                  - KAFKA_BROKER_ID=1
                                  - KAFKA_MIN_INSYNC_REPLICAS=2
                                  - KAFKA_DEFAULT_REPLICATION_FACTOR=3
                                  - KAFKA_ZOOKEEPER_CONNECT=fabric_zookeeper1:2181,fabric_zookeeper2:2182,fabric_zookeeper3:2183
                                  - KAFKA_MESSAGE_MAX_BYTES=103809024
                                  - KAFKA_REPLICA_FETCH_MAX_BYTES=103809024
                                  - KAFKA_UNCLEAN_LEADER_ELECTION_ENABLE=false
                                  - KAFKA_LOG_RETENTION_MS=-1
                                  - KAFKA_HEAP_OPTS=-Xmx256M -Xms128M
                                networks:
                                  - test
                                depends_on:
                                  - fabric_zookeeper1
                                  - fabric_zookeeper2
                                  - fabric_zookeeper3
                                deploy:
                                  mode: replicated
                                  replicas: 1
                                  restart_policy:
                                    condition: on-failure
                                    delay: 5s
                                    max_attempts: 3
                                  update_config:
                                    parallelism: 1
                                    delay: 10s
                            
                              kafka2:
                                image: hyperledger/fabric-kafka:0.4.10
                                ports:
                                  - "9092:9092"
                                environment:
                                  - KAFKA_BROKER_ID=2
                                  - KAFKA_MIN_INSYNC_REPLICAS=2
                                  - KAFKA_DEFAULT_REPLICATION_FACTOR=3
                                  - KAFKA_ZOOKEEPER_CONNECT=fabric_zookeeper1:2181,fabric_zookeeper2:2182,fabric_zookeeper3:2183
                                  - KAFKA_MESSAGE_MAX_BYTES=103809024
                                  - KAFKA_REPLICA_FETCH_MAX_BYTES=103809024
                                  - KAFKA_UNCLEAN_LEADER_ELECTION_ENABLE=false
                                  - KAFKA_LOG_RETENTION_MS=-1
                                  - KAFKA_HEAP_OPTS=-Xmx256M -Xms128M
                                networks:
                                  - test
                                depends_on:
                                  - fabric_zookeeper1
                                  - fabric_zookeeper2
                                  - fabric_zookeeper3
                                deploy:
                                  mode: replicated
                                  replicas: 1
                                  restart_policy:
                                    condition: on-failure
                                    delay: 5s
                                    max_attempts: 3
                                  update_config:
                                    parallelism: 1
                                    delay: 10s
                            
                              kafka3:
                                image: hyperledger/fabric-kafka:0.4.10
                                ports:
                                  - "9093:9092"
                                environment:
                                  - KAFKA_BROKER_ID=3
                                  - KAFKA_MIN_INSYNC_REPLICAS=2
                                  - KAFKA_DEFAULT_REPLICATION_FACTOR=3
                                  - KAFKA_ZOOKEEPER_CONNECT=fabric_zookeeper1:2181,fabric_zookeeper2:2182,fabric_zookeeper3:2183
                                  - KAFKA_MESSAGE_MAX_BYTES=103809024
                                  - KAFKA_REPLICA_FETCH_MAX_BYTES=103809024
                                  - KAFKA_UNCLEAN_LEADER_ELECTION_ENABLE=false
                                  - KAFKA_LOG_RETENTION_MS=-1
                                  - KAFKA_HEAP_OPTS=-Xmx256M -Xms128M
                                networks:
                                  - test
                                depends_on:
                                  - fabric_zookeeper1
                                  - fabric_zookeeper2
                                  - fabric_zookeeper3
                                deploy:
                                  mode: replicated
                                  replicas: 1
                                  restart_policy:
                                    condition: on-failure
                                    delay: 5s
                                    max_attempts: 3
                                  update_config:
                                    parallelism: 1
                                    delay: 10s
                            
                              kafka4:
                                image: hyperledger/fabric-kafka:0.4.10
                                ports:
                                  - "9094:9092"
                                environment:
                                  - KAFKA_BROKER_ID=4
                                  - KAFKA_MIN_INSYNC_REPLICAS=2
                                  - KAFKA_DEFAULT_REPLICATION_FACTOR=3
                                  - KAFKA_ZOOKEEPER_CONNECT=fabric_zookeeper1:2181,fabric_zookeeper2:2182,fabric_zookeeper3:2183
                                  - KAFKA_MESSAGE_MAX_BYTES=103809024
                                  - KAFKA_REPLICA_FETCH_MAX_BYTES=103809024
                                  - KAFKA_UNCLEAN_LEADER_ELECTION_ENABLE=false
                                  - KAFKA_LOG_RETENTION_MS=-1
                                  - KAFKA_HEAP_OPTS=-Xmx256M -Xms128M
                                networks:
                                  - test
                                depends_on:
                                  - fabric_zookeeper1
                                  - fabric_zookeeper2
                                  - fabric_zookeeper3
                                deploy:
                                  mode: replicated
                                  replicas: 1
                                  restart_policy:
                                    condition: on-failure
                                    delay: 5s
                                    max_attempts: 3
                                  update_config:
                                    parallelism: 1
                                    delay: 10s
                            
                            networks:
                              test:
                                driver: overlay
                         "#;

    #[test]
    fn test_self() {
        let yaml1 = Yaml::new(DATA).unwrap();
        let yaml2 = Yaml::new(DATA.to_string()).unwrap();
        let yaml3 = Yaml::new(DATA.as_bytes()).unwrap();
        let yaml4 = Yaml::new(DATA.as_bytes().to_vec()).unwrap();
        println!("yaml1 to string = {}", yaml1.to_string());
        println!("yaml2 to string = {}", yaml2.to_string());
        println!("yaml3 to string = {}", yaml3.to_string());
        println!("yaml4 to string = {}", yaml4.to_string());
        println!("yaml1 to slice = {:#?}", String::from_utf8(yaml1.to_vec()))
    }

    #[test]
    fn test_obj1() {
        let yaml = Yaml::new(USER).unwrap();
        println!("yaml = {:#?}", yaml.to_string());
        let user: User = yaml.to_object().unwrap();
        println!("user = {:#?}", user);
        let u1: User = Yaml::string_2_obj(yaml.to_string().as_str()).unwrap();
        println!("user = {:#?}", u1);
        let u2: User = Yaml::bytes_2_obj(yaml.to_vec().as_slice()).unwrap();
        println!("user = {:#?}", u2);
        let u3: User = Yaml::value_2_obj(yaml.value()).unwrap();
        println!("user = {:#?}", u3);
    }

    #[test]
    fn test_obj2() {
        let yaml = Yaml::new(OBJECT).unwrap();
        println!("yaml = {:#?}", yaml.to_string());
    }

    #[test]
    fn test_object_exec() {
        let yaml = Yaml::new(GET).unwrap();
        println!("string = {}", yaml.get_string("string").unwrap());
        println!("u64 = {}", yaml.get_u64("u64").unwrap());
        println!("i64 = {}", yaml.get_i64("i64").unwrap());
        println!("f64 = {}", yaml.get_f64("f64").unwrap());
        println!("bool = {}", yaml.get_bool("bool").unwrap());
        println!();
        println!("string = {}", yaml.is_string("string"));
        println!("u64 = {}", yaml.is_u64("u64"));
        println!("i64 = {}", yaml.is_i64("i64"));
        println!("f64 = {}", yaml.is_f64("f64"));
        println!("bool = {}", yaml.is_bool("bool"));
        println!();
        println!("string = {}", yaml.is_u64("string"));
        println!("u64 = {}", yaml.is_i64("u64"));
        println!("i64 = {}", yaml.is_f64("i64"));
        println!("f64 = {}", yaml.is_bool("f64"));
        println!("bool = {}", yaml.is_string("bool"));
        println!();
        let object = yaml.get_object("object").unwrap();
        println!("object string = {}", object.get_string("string").unwrap());
        println!("object u64 = {}", object.get_u64("u64").unwrap());
        println!("object i64 = {}", object.get_i64("i64").unwrap());
        println!("object f64 = {}", object.get_f64("f64").unwrap());
        println!("object bool = {}", object.get_bool("bool").unwrap());
    }

    #[test]
    fn test_array_self() {
        let array1 = Yaml::new(ARRAYS).unwrap();
        let array2 = Yaml::new(ARRAYS.to_string()).unwrap();
        let array3 = Yaml::new(ARRAYS.as_bytes()).unwrap();
        let array4 = Yaml::new(ARRAYS.as_bytes().to_vec()).unwrap();
        println!("array1 to string = {}", array1.to_string());
        println!("array2 to string = {}", array2.to_string());
        println!("array3 to string = {}", array3.to_string());
        println!("array4 to string = {}", array4.to_string());
        println!(
            "array1 to slice = {:#?}",
            String::from_utf8(array1.to_vec())
        )
    }

    #[test]
    fn test_array_obj() {
        let array = YamlArray::new(ARRAY_OBJECT).unwrap();
        let users: Vec<User> = array.to_object().unwrap();
        println!("user = {:#?}", users);
    }

    #[test]
    fn test_array1() {
        let yaml = Yaml::new(ARRAY).unwrap();
        println!("string = {}", yaml.get_string("string").unwrap());
        println!("u64 = {}", yaml.get_u64("u64").unwrap());
        println!("i64 = {}", yaml.get_i64("i64").unwrap());
        println!("f64 = {}", yaml.get_f64("f64").unwrap());
        println!("bool = {}", yaml.get_bool("bool").unwrap());
        let array = yaml.get_array("array1").unwrap();
        let object = array.get_object(0).unwrap();
        println!("object string = {}", object.get_string("string").unwrap());
        println!("object u64 = {}", object.get_u64("u64").unwrap());
        println!("object i64 = {}", object.get_i64("i64").unwrap());
        println!("object f64 = {}", object.get_f64("f64").unwrap());
        println!("object bool = {}", object.get_bool("bool").unwrap());
        let array = object.get_array("array").unwrap();
        println!("array 0 = {}", array.get_string(0).unwrap());
    }

    #[test]
    fn test_array2() {
        let array = YamlArray::new(ARRAYS).unwrap();
        let yaml = array.get_object(0).unwrap();
        println!("string = {}", yaml.get_string("string").unwrap());
        println!("u64 = {}", yaml.get_u64("u64").unwrap());
        println!("i64 = {}", yaml.get_i64("i64").unwrap());
        println!("f64 = {}", yaml.get_f64("f64").unwrap());
        println!("bool = {}", yaml.get_bool("bool").unwrap());
        let array = yaml.get_array("array").unwrap();
        println!("array 0 = {}", array.get_string(0).unwrap());
    }

    #[test]
    fn test_array3() {
        let array = YamlArray::new(ARRAYS).unwrap();
        let yaml = array.get_object(3).unwrap();
        println!("string = {}", yaml.get_string("string").unwrap());
        println!("u64 = {}", yaml.get_u64("u64").unwrap());
        println!("i64 = {}", yaml.get_i64("i64").unwrap());
        println!("f64 = {}", yaml.get_f64("f64").unwrap());
        println!("bool = {}", yaml.get_bool("bool").unwrap());
        let array = yaml.get_array("array").unwrap();
        println!("array 0 = {}", array.get_f64(0).unwrap());
        println!("array 0 = {}", array.get_string(1).unwrap());
        println!("array 0 = {}", array.get_u64(2).unwrap());
        println!("array 0 = {}", array.get_bool(3).unwrap());
        println!("array 0 = {}", array.get_i64(4).unwrap());
    }

    #[test]
    fn test_out() {
        let user = User {
            name: "1".to_string(),
            age: 2,
            blog: "3".to_string(),
            addr: "4".to_string(),
        };
        println!("object to string = {}", Yaml::obj_2_string(&user).unwrap());
        println!(
            "object to string = {:#?}",
            String::from_utf8(Yaml::obj_2_bytes(&user).unwrap())
        );
        println!("object = {}", Yaml::object(&user).unwrap().to_string());
    }
}
