#[cfg(test)]
mod index_test {
    use std::error::Error;
    use std::sync::{Arc, RwLock};

    use crate::engine::siam::index::Index;
    use crate::engine::siam::mem::node::Node;
    use crate::engine::siam::mem::seed::Seed;
    use crate::engine::traits::{TDescription, TIndex, TSeed};
    use crate::utils::comm::{Category, IndexMold};
    use comm::cryptos::hash::md516;

    fn obtain_index() -> Index<Node> {
        Index::create(
            String::from("database_id"),
            String::from("view_id"),
            String::from("1"),
            String::from("1"),
            false,
            Node::create_root(),
            Category::Memory,
            IndexMold::String,
        )
    }

    #[test]
    fn create_index() {
        let index = obtain_index();
        // println!("index is {:#?}", index);
        println!("index id = {}", index.id());
        println!("index index_name = {}", index.name());
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
            IndexMold::String,
        );
        let d = index.description();
        println!(
            "index = {}, {}, {}, {}",
            index.id(),
            index.is_primary(),
            index.name(),
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
            IndexMold::String,
        );
        let d1 = index1.description();
        println!("d1 = {:#?}", d1);
        println!(
            "index1 = {}, {}, {}, {}",
            index1.id(),
            index1.is_primary(),
            index1.name(),
            index1.create_time().num_nanoseconds().unwrap().to_string()
        );
        index1.recover(d).unwrap();
        println!(
            "index1 = {}, {}, {}, {}",
            index1.id(),
            index1.is_primary(),
            index1.name(),
            index1.create_time().num_nanoseconds().unwrap().to_string()
        );
    }

    #[test]
    fn put_get() {
        let index = obtain_index();
        let key = "test".to_string();
        let seed = Arc::new(RwLock::new(Seed::create(md516(key.clone()))));
        seed.write().unwrap().save("1".as_bytes().to_vec()).unwrap();
        index.put(key.clone(), seed, false).unwrap();
        let irg = index.get(key.clone());
        match irg {
            Ok(seed) => println!("u1 is {:#?}", seed),
            Err(ie) => println!("res1 is {:#?}", ie.source().unwrap().to_string()),
        }
        let irr = index.remove(key.clone());
        match irr {
            Ok(seed) => println!("u2 is {:#?}", seed),
            Err(ie) => println!("res2 is {:#?}", ie.source().unwrap().to_string()),
        }
        let irg = index.get(key.clone());
        match irg {
            Ok(seed) => println!("u3 is {:#?}", seed),
            Err(ie) => println!("res3 is {:#?}", ie.source().unwrap().to_string()),
        }
    }
}
