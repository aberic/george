#[cfg(test)]
mod log_test {
    use crate::set_log;

    #[test]
    fn logs_macro() {
        set_log(
            String::from("log"),
            String::from("src/test"),
            1024,
            7,
            String::from("trace"),
        );
        trace!("Hello, macros!");
        debug!("Hello, macros!");
        info!("Hello, macros!");
        warn!("Hello, macros!");
        error!("Hello, macros!");
    }
}
