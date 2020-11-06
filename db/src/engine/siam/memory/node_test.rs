#[cfg(test)]
mod node_test {
    use std::sync::{Arc, RwLock};

    use crate::engine::siam::comm::add_child_node;
    use crate::engine::siam::memory::node::Node;
    use crate::engine::siam::memory::seed::Seed;
    use crate::engine::siam::traits::TNode;
    use crate::engine::traits::TSeed;
    use crate::utils::comm::LevelType;
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
    fn create_node_test() {
        let n: Arc<Node> = Node::create_node(2);
        println!("node is {:#?}", n);
        println!("node degree_index = {}", n.degree_index());
        println!("node nodes = {:#?}", n.nodes());
        println!();
    }

    #[test]
    fn create_leaf_test() {
        let n: Arc<Node> = Node::create_leaf(2);
        println!("node is {:#?}", n);
        println!("node degree_index = {}", n.degree_index());
        println!("node nodes = {:#?}", n.nodes());
        println!();
    }

    #[test]
    fn add_child_node_test() {
        let pre_n: Arc<Node> = Node::create_root();
        let n: Arc<Node> = Node::create_node(2);
        add_child_node(&*pre_n, n.clone());
        let nc1 = Node::create_node(13);
        let nc2 = Node::create_node(20);
        let nc3 = Node::create_node(4);
        add_child_node(&*n, nc1);
        add_child_node(&*n, nc2);
        add_child_node(&*n, nc3);
        // let pn = n.clone().pre_node().unwrap();
        // let pv = pn.clone().nodes().unwrap().read().unwrap()[0].clone();
        // let v = pv.clone().nodes().unwrap().read().unwrap().clone();
        for node in n.clone().nodes().unwrap().read().unwrap().iter() {
            println!("node degree = {}", node.degree_index())
        }
    }

    #[test]
    fn put_get_32() {
        let root: Arc<Node> = Node::create_root();
        let key = "test".to_string();
        let seed = Arc::new(RwLock::new(Seed::create(md516(key.clone()))));
        seed.write().unwrap().save("1".as_bytes().to_vec());
        root.put(key.clone(), seed, false, 0, LevelType::Small)
            .unwrap();
        let irg = root.get(key.clone(), 0, LevelType::Small);
        match irg {
            Ok(seed) => println!("u is {:#?}", seed),
            Err(ie) => println!("res is {:#?}", ie.source().unwrap().to_string()),
        }
        let irg = root.get(key.clone(), 0, LevelType::Large);
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
        seed.write().unwrap().save("1".as_bytes().to_vec());
        root.put(key.clone(), seed, false, 0, LevelType::Large)
            .unwrap();
        let irg = root.get(key.clone(), 0, LevelType::Large);
        match irg {
            Ok(seed) => println!("u is {:#?}", seed),
            Err(ie) => println!("res is {:#?}", ie.source().unwrap().to_string()),
        }
        let irg = root.get(key.clone(), 0, LevelType::Small);
        match irg {
            Ok(seed) => println!("u is {:#?}", seed),
            Err(ie) => println!("res is {:#?}", ie.source().unwrap().to_string()),
        }
    }
}
