#[cfg(test)]
mod env {
    use crate::env::get;

    # [test]
    fn env_get_test () {
        println!("env = {}", get("env", "hello"));
        println!("env = {}", get("GOPATH", "hello"));
    }
}