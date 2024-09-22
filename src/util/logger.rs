use colored::Colorize;

pub enum LogLevel {
    Debug = 0,
    Info = 10,
    Warn = 20,
    Error = 30,
}

static mut LOG_LEVEL: i32 = LogLevel::Info as i32;

fn __get_log_level() -> i32 {
    unsafe { LOG_LEVEL }
}

fn __set_log_level(level: LogLevel) {
    unsafe {
        LOG_LEVEL = level as i32;
    }
}

pub fn __log(s: &str) {
    if __get_log_level() <= LogLevel::Debug as i32 {
        println!("{} {}", "[LOG]".blue(), s);
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ($crate::util::logger::__log(&format!($($arg)*)))
}

pub fn __info(s: &str) {
    if __get_log_level() <= LogLevel::Info as i32 {
        println!("{} {}", "[INFO]".green(), s);
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ($crate::util::logger::__info(&format!($($arg)*)))
}

pub fn __warn(s: &str) {
    if __get_log_level() <= LogLevel::Warn as i32 {
        eprintln!("{} {}", "[WARN]".yellow(), s);
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ($crate::util::logger::__warn(&format!($($arg)*)))
}

pub fn __err(s: &str) {
    if __get_log_level() <= LogLevel::Error as i32 {
        eprintln!("{} {}", "[ERROR]".red(), s);
    }
}

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => ($crate::util::logger::__err(&format!($($arg)*)))
}

pub fn get_log_level() -> LogLevel {
    match __get_log_level() {
        0 => LogLevel::Debug,
        10 => LogLevel::Info,
        20 => LogLevel::Warn,
        30 => LogLevel::Error,
        _ => LogLevel::Debug,
    }
}

pub fn set_log_level(level: LogLevel) {
    __set_log_level(level);
}
