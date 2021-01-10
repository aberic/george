#[cfg(test)]
mod node_test {
    use crate::engine::siam::doc32::node::Node;
    use crate::engine::siam::traits::TNode;
    use std::sync::Arc;

    #[test]
    fn create_root_test() {
        let n: Arc<Node> = Node::create_root(
            "database".to_string(),
            "view".to_string(),
            "index".to_string(),
        );
        println!("node is {:#?}", n);
        println!("node degree_index = {}", n.degree_index());
        println!("node nodes = {:#?}", n.nodes());
        println!();
    }
}
