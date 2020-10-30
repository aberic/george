#[cfg(test)]
mod index_test {
    use std::error::Error;
    use std::sync::{Arc, RwLock};

    use crate::engine::siam::index::Index;
    use crate::engine::siam::memory::node::Node;
    use crate::engine::siam::memory::seed::Seed;
    use crate::engine::traits::{TDescription, TIndex};
    use crate::utils::comm::{Category, LevelType};

    fn obtain_index() -> Index<Node> {
        Index::create(
            String::from("database_id"),
            String::from("view_id"),
            String::from("1"),
            String::from("1"),
            false,
            Node::create_root(),
            Category::Memory,
            LevelType::Large,
        )
    }

    #[test]
    fn create_index() {
        let index = obtain_index();
        // println!("index is {:#?}", index);
        println!("index id = {}", index.id());
        println!("index key_structure = {}", index.key_structure());
        println!("index is_primary = {}", index.is_primary());
        println!();
    }

    #[test]
    fn description() {
        let mut index: Index<Node> = Index::create(
            String::from("database_id"),
            String::from("view_id"),
            String::from("1"),
            String::from("1"),
            false,
            Node::create_root(),
            Category::Memory,
            LevelType::Large,
        );
        let d = index.description();
        println!(
            "index = {}, {}, {}, {}",
            index.id(),
            index.is_primary(),
            index.key_structure(),
            index.create_time().num_nanoseconds().unwrap().to_string()
        );
        println!("d = {:#?}", d);

        let mut index1: Index<Node> = Index::create(
            String::from("database_id"),
            String::from("view_id"),
            String::from("2"),
            String::from("2"),
            true,
            Node::create_root(),
            Category::Memory,
            LevelType::Small,
        );
        let d1 = index1.description();
        println!("d1 = {:#?}", d1);
        println!(
            "index1 = {}, {}, {}, {}",
            index1.id(),
            index1.is_primary(),
            index1.key_structure(),
            index1.create_time().num_nanoseconds().unwrap().to_string()
        );
        index1.recover(d).unwrap();
        println!(
            "index1 = {}, {}, {}, {}",
            index1.id(),
            index1.is_primary(),
            index1.key_structure(),
            index1.create_time().num_nanoseconds().unwrap().to_string()
        );
    }

    #[test]
    fn put() {
        let index = obtain_index();
        let seed = Arc::new(RwLock::new(Seed::create(String::from("1"))));
        index.put("test".to_string(), seed, false).unwrap();
        let irg = index.get("test".to_string());
        match irg {
            Ok(seed) => println!("u is {:#?}", seed),
            Err(ie) => println!("res is {:#?}", ie.source().unwrap().to_string()),
        }
    }
}
