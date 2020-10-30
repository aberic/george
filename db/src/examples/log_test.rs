#[cfg(test)]
mod log_test {

    use logs::set_log;

    #[test]
    fn logs() {
        set_log(
            String::from("db"),
            String::from("src/test"),
            1024,
            7,
            String::from("trace"),
        );
        log::trace!("Hello, world!");
        log::debug!("Hello, world!");
        log::info!("Hello, world!");
        log::warn!("Hello, world!");
        log::error!("Hello, world!");
    }
}
