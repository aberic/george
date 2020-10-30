#[cfg(test)]
mod config {
    use crate::utils::deploy::{GLOBAL_CONFIG, init};

    #[test]
    fn init_test() {
        init("src/examples/conf.yaml".to_string());
        println!("config = {:#?}", GLOBAL_CONFIG);
    }
}
