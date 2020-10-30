#[cfg(test)]
mod node_test {
    use std::sync::Arc;

    use crate::engine::siam::comm::add_child_node;
    use crate::engine::siam::memory::node::Node;
    use crate::engine::siam::traits::TNode;

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
}
