#[cfg(test)]
mod node_test {
    use std::sync::{Arc, RwLock};

    use crate::engine::siam::mem::node::Node;
    use crate::engine::siam::mem::seed::Seed;
    use crate::engine::siam::traits::TNode;
    use crate::engine::traits::TSeed;
    use comm::cryptos::hash::md516;
    use std::error::Error;

    #[test]
    fn create_root_test() {
        let n: Arc<Node> = Node::create_root();
        println!("node is {:#?}", n);
        println!("node degree_index = {}", n.degree_index());
        println!("node nodes = {:#?}", n.nodes());
        println!();
    }

    #[test]
    fn put_get_32() {
        let root: Arc<Node> = Node::create_root();
        let key = "test".to_string();
        let seed = Arc::new(RwLock::new(Seed::create(md516(key.clone()))));
        seed.write().unwrap().save("1".as_bytes().to_vec()).unwrap();
        root.put(key.clone(), seed, false, 0).unwrap();
        let irg = root.get(key.clone());
        match irg {
            Ok(seed) => println!("u is {:#?}", seed),
            Err(ie) => println!("res is {:#?}", ie.source().unwrap().to_string()),
        }
        let irg = root.get(key.clone());
        match irg {
            Ok(seed) => println!("u is {:#?}", seed),
            Err(ie) => println!("res is {:#?}", ie.source().unwrap().to_string()),
        }
    }

    #[test]
    fn put_get_64() {
        let root: Arc<Node> = Node::create_root();
        let key = "test".to_string();
        let seed = Arc::new(RwLock::new(Seed::create(md516(key.clone()))));
        seed.write().unwrap().save("1".as_bytes().to_vec()).unwrap();
        root.put(key.clone(), seed, false, 0).unwrap();
        let irg = root.get(key.clone());
        match irg {
            Ok(seed) => println!("u is {:#?}", seed),
            Err(ie) => println!("res is {:#?}", ie.source().unwrap().to_string()),
        }
        let irg = root.get(key.clone());
        match irg {
            Ok(seed) => println!("u is {:#?}", seed),
            Err(ie) => println!("res is {:#?}", ie.source().unwrap().to_string()),
        }
    }
}
