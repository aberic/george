use crate::utils::deploy::{init_config, GLOBAL_CONFIG};

#[test]
fn init_test() {
    init_config("src/examples/conf.yaml".to_string());
    println!("config = {:#?}", GLOBAL_CONFIG);
}
