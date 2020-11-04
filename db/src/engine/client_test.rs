#[cfg(test)]
mod engine_test {
    use std::error::Error;

    use crate::engine::client::GLOBAL_CLIENT;
    use crate::utils::comm::{Category, IndexType, LevelType};

    #[test]
    fn put_memory() {
        create_database("database", "comment", 1);
        create_database("database", "comment", 2);
        create_database("database1", "comment", 3);
        create_view(
            "database",
            "view",
            "comment",
            IndexType::Siam,
            Category::Memory,
            LevelType::Small,
            1,
        );
        create_view(
            "database",
            "view",
            "comment",
            IndexType::Siam,
            Category::Memory,
            LevelType::Small,
            2,
        );
        create_view(
            "database",
            "view1",
            "comment",
            IndexType::Siam,
            Category::Memory,
            LevelType::Small,
            3,
        );
        create_index("database", "view", "index", false, 1);
        create_index("database", "view", "index", false, 2);
        put("database", "view", "md516", "database1 tValue", 1);
        get("database", "view", "md516", 1);
        put("database", "view", "md516", "database2 tValue", 2);
        get("database", "view", "md516", 2);
        set("database", "view", "md516", "database3 tValue", 3);
        get("database", "view", "md516", 3);
        modify_view("database", "view", "view_new", 4);
        get("database", "view_new", "md516", 4);
        put("database", "view_new", "md5161", "database5 tValue", 5);
        put("database", "view_new", "md5162", "database6 tValue", 6);
        put("database", "view_new", "md5163", "database7 tValue", 7);
        put("database", "view_new", "md5164", "database8 tValue", 8);
        get("database", "view_new", "md5161", 5);
        get("database", "view_new", "md5162", 7);
        get("database", "view_new", "md5163", 8);
        get("database", "view_new", "md5164", 9);
        get("database", "view_new", "md5165", 10);
        get("database", "view", "md516", 11);
        modify_database("database", "database1", 13);
        get("database1", "view_new", "md516", 12);
        get("database1", "view_new", "md516", 14);
        get("database", "view_new", "md516", 15);
    }

    #[test]
    fn put_document() {
        create_database("database", "comment", 1);
        create_database("database", "comment", 2);
        create_database("database1", "comment", 3);
        create_view(
            "database",
            "view_doc",
            "comment",
            IndexType::Siam,
            Category::Document,
            LevelType::Small,
            1,
        );
        create_view(
            "database",
            "view_doc",
            "comment",
            IndexType::Siam,
            Category::Document,
            LevelType::Small,
            2,
        );
        create_view(
            "database",
            "view_doc1",
            "comment",
            IndexType::Siam,
            Category::Document,
            LevelType::Small,
            3,
        );
        create_index("database", "view_doc", "index", false, 1);
        create_index("database", "view_doc", "index", false, 2);
        put("database", "view_doc", "md516", "database1 tValue", 1);
        get("database", "view_doc", "md516", 1);
        put("database", "view_doc", "md516", "database2 tValue", 2);
        get("database", "view_doc", "md516", 2);
        set("database", "view_doc", "md516", "database3 tValue", 3);
        get("database", "view_doc", "md516", 3);
        modify_view("database", "view_doc", "view_doc_new", 4);
        get("database", "view_doc_new", "md516", 4);
        put("database", "view_doc_new", "md5161", "database5 tValue", 5);
        put("database", "view_doc_new", "md5162", "database6 tValue", 6);
        put("database", "view_doc_new", "md5163", "database7 tValue", 7);
        put("database", "view_doc_new", "md5164", "database8 tValue", 8);
        get("database", "view_doc_new", "md5161", 5);
        get("database", "view_doc_new", "md5162", 6);
        get("database", "view_doc_new", "md5163", 7);
        get("database", "view_doc_new", "md5164", 8);
        get("database", "view_doc_new", "md5165", 9);
        get("database", "view_doc", "md516", 10);
        get("database1", "view_doc_new", "md516", 11);
        modify_database("database", "database1", 1);
        modify_database("database", "database2", 2);
        get("database2", "view_doc_new", "md516", 12);
        get("database", "view_doc_new", "md516", 13);
    }

    #[test]
    fn put_document2() {
        create_database("database_test", "comment", 1);
        create_view(
            "database_test",
            "view_test_doc",
            "comment",
            IndexType::Siam,
            Category::Document,
            LevelType::Small,
            1,
        );
        create_index("database_test", "view_test_doc", "index_test", false, 1);
        put("database_test", "view_test_doc", "key", "value1", 1);
        get("database_test", "view_test_doc", "key", 2);
        set("database_test", "view_test_doc", "key", "value2", 3);
        get("database_test", "view_test_doc", "key", 4);
    }

    fn create_database(database_name: &str, database_comment: &str, position: usize) {
        create_database_string(
            database_name.to_string(),
            database_comment.to_string(),
            position,
        )
    }

    fn create_database_string(database_name: String, database_comment: String, position: usize) {
        match GLOBAL_CLIENT.create_database(database_name, database_comment) {
            Err(err) => println!("create_database{} database_test = {}", position, err),
            _ => {}
        }
    }

    fn create_view(
        database_name: &str,
        view_name: &str,
        view_comment: &str,
        index_type: IndexType,
        view_category: Category,
        view_level: LevelType,
        position: usize,
    ) {
        match GLOBAL_CLIENT.create_view(
            database_name.to_string(),
            view_name.to_string(),
            view_comment.to_string(),
            index_type,
            view_category,
            view_level,
        ) {
            Err(err) => println!("create_view{} view_test_doc = {}", position, err),
            _ => {}
        }
    }

    fn create_index(
        database_name: &str,
        view_name: &str,
        key_structure: &str,
        primary: bool,
        position: usize,
    ) {
        match GLOBAL_CLIENT.create_index(
            database_name.to_string(),
            view_name.to_string(),
            key_structure.to_string(),
            primary,
        ) {
            Err(err) => println!("create_index{} {} = {}", position, key_structure, err),
            _ => {}
        }
    }

    fn put(database_name: &str, view_name: &str, key: &str, value: &str, position: usize) {
        match GLOBAL_CLIENT.put(
            database_name.to_string(),
            view_name.to_string(),
            key.to_string(),
            value.to_string().into_bytes(),
        ) {
            Err(ie) => println!(
                "put{} error is {:#?}",
                position,
                ie.source().unwrap().to_string()
            ),
            _ => {}
        }
    }

    fn set(database_name: &str, view_name: &str, key: &str, value: &str, position: usize) {
        match GLOBAL_CLIENT.set(
            database_name.to_string(),
            view_name.to_string(),
            key.to_string(),
            value.to_string().into_bytes(),
        ) {
            Err(ie) => println!(
                "put{} error is {:#?}",
                position,
                ie.source().unwrap().to_string()
            ),
            _ => {}
        }
    }

    fn get(database_name: &str, view_name: &str, key: &str, position: usize) {
        match GLOBAL_CLIENT.get(
            database_name.to_string(),
            view_name.to_string(),
            key.to_string(),
        ) {
            Ok(vu8) => println!(
                "get{} is {:#?}",
                position,
                String::from_utf8(vu8).unwrap().as_str()
            ),
            Err(ie) => println!("get{} is {:#?}", position, ie.source().unwrap().to_string()),
        }
    }

    fn modify_view(database_name: &str, view_old_name: &str, view_new_name: &str, position: usize) {
        match GLOBAL_CLIENT.modify_view(
            database_name.to_string(),
            view_old_name.to_string(),
            view_new_name.to_string(),
        ) {
            Err(ie) => println!(
                "modify view {} is {:#?}",
                position,
                ie.source().unwrap().to_string()
            ),
            _ => {}
        }
    }

    fn modify_database(database_old_name: &str, database_new_name: &str, position: usize) {
        match GLOBAL_CLIENT
            .modify_database(database_old_name.to_string(), database_new_name.to_string())
        {
            Err(ie) => println!(
                "modify database {} is {:#?}",
                position,
                ie.source().unwrap().to_string()
            ),
            _ => {}
        }
    }
}
