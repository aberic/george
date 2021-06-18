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
mod comm {
    use crate::impls::db::database::Database;
    use crate::impls::utils::Comm;
    use comm::Time;
    use protobuf::Message;

    #[test]
    fn test_parse() {
        let t1 = "   asd   asd asd\n
        \n
        asd  asd  asd\n
           "
        .to_string();
        println!("res = {}", Comm::parse_str(t1))
    }

    #[test]
    fn test_message() {
        let mut item = Database::new();
        item.set_name("test".to_string());
        item.set_comment("comment".to_string());
        item.set_create_time(Comm::proto_time_2_grpc_timestamp(Time::now()));
        let res = item.write_to_bytes().unwrap();
        println!("res = \n{:#?}", res);
        let mut db = Database::new();
        db.merge_from_bytes(res.as_slice()).unwrap();
        println!("db = \n{:#?}", db);
    }
}
