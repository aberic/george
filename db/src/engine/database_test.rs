#[cfg(test)]
mod database_test {
    use std::error::Error;

    use crate::engine::database::Database;
    use crate::engine::traits::TDescription;
    use crate::utils::comm::{Category, IndexType, LevelType};

    #[test]
    fn create() {
        let database: Database = Database::create(String::from("name"), String::from("comment"));
        println!("database id = {}", database.id());
        println!("database name = {}", database.name());
        println!("database comment = {}", database.comment());
        println!("database create_time = {:#?}", database.create_time());
        println!();
    }

    #[test]
    fn description() {
        let mut database: Database =
            Database::create(String::from("name"), String::from("comment"));
        let d = database.description();
        println!(
            "database = {}, {}, {}, {}",
            database.id(),
            database.name(),
            database.comment(),
            database
                .create_time()
                .num_nanoseconds()
                .unwrap()
                .to_string()
        );
        println!("d = {:#?}", d);

        let mut database1: Database =
            Database::create(String::from("name1"), String::from("comment1"));
        let d1 = database1.description();
        println!("d1 = {:#?}", d1);
        println!(
            "database1 = {}, {}, {}, {}",
            database1.id(),
            database1.name(),
            database1.comment(),
            database1
                .create_time()
                .num_nanoseconds()
                .unwrap()
                .to_string()
        );
        database1.recover(d).unwrap();
        println!(
            "database1 = {}, {}, {}, {}",
            database1.id(),
            database1.name(),
            database1.comment(),
            database1
                .create_time()
                .num_nanoseconds()
                .unwrap()
                .to_string()
        );
    }

    #[test]
    fn put() {
        let mut database: Database =
            Database::create(String::from("name"), String::from("comment"));
        database
            .create_view(
                String::from("name"),
                String::from("comment"),
                IndexType::Siam,
                Category::Memory,
                LevelType::Small,
            )
            .unwrap();
        match database.create_view(
            String::from("name"),
            String::from("comment"),
            IndexType::Siam,
            Category::Memory,
            LevelType::Small,
        ) {
            Err(err) => println!("create_view = {}", err),
            _ => {}
        }
        database
            .create_index(String::from("name"), String::from("1"), false)
            .unwrap();
        match database.create_index(String::from("name"), String::from("1"), false) {
            Err(err) => println!("create_index = {}", err),
            _ => {}
        }
        let irp = database.put(
            String::from("name"),
            String::from("md516"),
            String::from("database1 tValue").into_bytes(),
        );
        match irp {
            Err(ie) => println!("res1 is {:#?}", ie.source().unwrap().to_string()),
            _ => {}
        }
        let irg = database.get(String::from("name"), String::from("md516"));
        match irg {
            Ok(vu8) => {
                // println!("u is {:#?}", vu8);
                println!("u1 is {:#?}", String::from_utf8(vu8).unwrap().as_str())
            }
            Err(ie) => println!("rlt1 is {:#?}", ie.source().unwrap().to_string()),
        }
        let irp = database.put(
            String::from("name"),
            String::from("md516"),
            String::from("database2 tValue").into_bytes(),
        );
        match irp {
            Err(ie) => println!("res2 is {:#?}", ie.source().unwrap().to_string()),
            _ => {}
        }
        let irg = database.get(String::from("name"), String::from("md516"));
        match irg {
            Ok(vu8) => {
                // println!("u is {:#?}", vu8);
                println!("u2 is {:#?}", String::from_utf8(vu8).unwrap().as_str())
            }
            Err(ie) => println!("rlt2 is {:#?}", ie.source().unwrap().to_string()),
        }
        let irp = database.set(
            String::from("name"),
            String::from("md516"),
            String::from("database3 tValue").into_bytes(),
        );
        match irp {
            Err(ie) => println!("res3 is {:#?}", ie.source().unwrap().to_string()),
            _ => {}
        }
        let irg = database.get(String::from("name"), String::from("md516"));
        match irg {
            Ok(vu8) => {
                // println!("u is {:#?}", vu8);
                println!("u3 is {:#?}", String::from_utf8(vu8).unwrap().as_str())
            }
            Err(ie) => println!("rlt3 is {:#?}", ie.source().unwrap().to_string()),
        }
        let irm = database.modify_view(String::from("name"), String::from("name_new"));
        match irm {
            Err(ie) => println!("res4 is {:#?}", ie.source().unwrap().to_string()),
            _ => {}
        }
        let irg = database.get(String::from("name_new"), String::from("md516"));
        match irg {
            Ok(vu8) => {
                // println!("u is {:#?}", vu8);
                println!("u4 is {:#?}", String::from_utf8(vu8).unwrap().as_str())
            }
            Err(ie) => println!("rlt4 is {:#?}", ie.source().unwrap().to_string()),
        }
        database
            .put(
                String::from("name_new"),
                String::from("md5161"),
                String::from("database5 tValue").into_bytes(),
            )
            .unwrap();
        database
            .put(
                String::from("name_new"),
                String::from("md5162"),
                String::from("database6 tValue").into_bytes(),
            )
            .unwrap();
        database
            .put(
                String::from("name_new"),
                String::from("md5163"),
                String::from("database7 tValue").into_bytes(),
            )
            .unwrap();
        database
            .put(
                String::from("name_new"),
                String::from("md5164"),
                String::from("database8 tValue").into_bytes(),
            )
            .unwrap();
        let irg = database.get(String::from("name_new"), String::from("md5161"));
        match irg {
            Ok(vu8) => {
                // println!("u is {:#?}", vu8);
                println!("u5 is {:#?}", String::from_utf8(vu8).unwrap().as_str())
            }
            Err(ie) => println!("rlt5 is {:#?}", ie.source().unwrap().to_string()),
        }
        let irg = database.get(String::from("name_new"), String::from("md5162"));
        match irg {
            Ok(vu8) => {
                // println!("u is {:#?}", vu8);
                println!("u7 is {:#?}", String::from_utf8(vu8).unwrap().as_str())
            }
            Err(ie) => println!("rlt7 is {:#?}", ie.source().unwrap().to_string()),
        }
        let irg = database.get(String::from("name_new"), String::from("md5163"));
        match irg {
            Ok(vu8) => {
                // println!("u is {:#?}", vu8);
                println!("u8 is {:#?}", String::from_utf8(vu8).unwrap().as_str())
            }
            Err(ie) => println!("rlt8 is {:#?}", ie.source().unwrap().to_string()),
        }
        let irg = database.get(String::from("name_new"), String::from("md5164"));
        match irg {
            Ok(vu8) => {
                // println!("u is {:#?}", vu8);
                println!("u9 is {:#?}", String::from_utf8(vu8).unwrap().as_str())
            }
            Err(ie) => println!("rlt9 is {:#?}", ie.source().unwrap().to_string()),
        }
        let irg = database.get(String::from("name_new"), String::from("md5165"));
        match irg {
            Ok(vu8) => {
                // println!("u is {:#?}", vu8);
                println!("u10 is {:#?}", String::from_utf8(vu8).unwrap().as_str())
            }
            Err(ie) => println!("rlt10 is {:#?}", ie.source().unwrap().to_string()),
        }
        let irg = database.get(String::from("name"), String::from("md516"));
        match irg {
            Ok(vu8) => {
                // println!("u is {:#?}", vu8);
                println!("u11 is {:#?}", String::from_utf8(vu8).unwrap().as_str())
            }
            Err(ie) => println!("rlt11 is {:#?}", ie.source().unwrap().to_string()),
        }
    }
}
