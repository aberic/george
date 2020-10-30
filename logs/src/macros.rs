#[macro_export]
macro_rules! trace {
    ($name:expr) => {
        log::trace!("{}", $name);
    };
}

#[macro_export]
macro_rules! debug {
    ($name:expr) => {
        log::debug!("{}", $name);
    };
}

#[macro_export]
macro_rules! info {
    ($name:expr) => {
        log::info!("{}", $name);
    };
}

#[macro_export]
macro_rules! warn {
    ($name:expr) => {
        log::warn!("{}", $name);
    };
}

#[macro_export]
macro_rules! error {
    ($name:expr) => {
        log::error!("{}", $name);
    };
}
