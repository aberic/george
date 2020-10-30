use std::sync::RwLock;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::json::JsonEncoder;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Handle;
use once_cell::sync::Lazy;

mod examples;
#[macro_use]
pub mod macros;
mod macros_test;

pub struct LogHandle {
    handle: Handle,
    /// 是否生产环境，在生产环境下控制台不会输出任何日志
    production: bool,
}

pub static GLOBAL_LOG: Lazy<RwLock<LogHandle>> = Lazy::new(|| {
    let service_name = "log".to_string();
    let dir = "src/test".to_string();
    let level = "debug".to_string();
    let file_max_size = 1024;
    let file_max_count = 7;
    let handle = LogHandle {
        handle: log4rs::init_config(log_config(
            service_name,
            dir,
            file_max_size,
            file_max_count,
            level,
        ))
        .unwrap(),
        production: false,
    };
    RwLock::new(handle)
});

/// 初始化日志
///
/// service_name 日志所服务的服务名称
///
/// log_dir 日志文件目录
///
/// log_file_max_size 每个日志文件保存的最大尺寸 单位：M
///
/// file_max_count 文件最多保存多少个
///
/// log_level 日志级别(debug/info/warn/Error/panic/fatal)
pub fn set_log(
    service_name: String,
    dir: String,
    file_max_size: u64,
    file_max_count: u32,
    level: String,
) {
    // log4rs::init_file("src/log4rs.yaml", Default::default()).unwrap();
    GLOBAL_LOG.write().unwrap().handle.set_config(log_config(
        service_name,
        dir,
        file_max_size,
        file_max_count,
        level,
    ))
}

/// 初始化日志
///
/// service_name 日志所服务的服务名称
///
/// log_dir 日志文件目录
///
/// log_file_max_size 每个日志文件保存的最大尺寸 单位：M
///
/// file_max_count 文件最多保存多少个
///
/// log_level 日志级别(debug/info/warn/Error/panic/fatal)
fn log_config(
    service_name: String,
    dir: String,
    file_max_size: u64,
    file_max_count: u32,
    level: String,
) -> Config {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%+)(local)} {l} {t} {m}{n}",
        )))
        .build();

    let files = RollingFileAppender::builder()
        .append(true)
        .encoder(Box::new(JsonEncoder::default()))
        .build(
            format!("{}/{}{}", dir.clone(), service_name, ".log"),
            Box::new(CompoundPolicy::new(
                Box::new(SizeTrigger::new(file_max_size * 1024 * 1024)),
                Box::new(
                    FixedWindowRoller::builder()
                        .build(
                            &*format!("{}{}", service_name, "-log-{}.log"),
                            file_max_count,
                        )
                        .unwrap(),
                ),
            )),
        )
        .unwrap();

    return Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("files", Box::new(files)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("files")
                .build(log_level(level)),
        )
        .unwrap();
}

fn log_level(level: String) -> LevelFilter {
    match level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Off,
    }
}
