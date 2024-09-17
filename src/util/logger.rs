use colored::Colorize;

use crate::singleton;

pub enum LogLevel {
    Debug = 0,
    Info = 10,
    Warn = 20,
    Error = 30,
}

singleton!(LogLevelConf, i32, LogLevel::Info as i32);

pub fn __log(s: &str) {
    if *LogLevelConf::get() <= LogLevel::Debug as i32 {
        println!("{} {}", "[LOG]".blue(), s);
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ($crate::util::logger::__log(&format!($($arg)*)))
}

pub fn __info(s: &str) {
    if *LogLevelConf::get() <= LogLevel::Info as i32 {
        println!("{} {}", "[INFO]".green(), s);
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ($crate::util::logger::__info(&format!($($arg)*)))
}

pub fn __warn(s: &str) {
    if *LogLevelConf::get() <= LogLevel::Warn as i32 {
        eprintln!("{} {}", "[WARN]".yellow(), s);
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ($crate::util::logger::__warn(&format!($($arg)*)))
}

pub fn __err(s: &str) {
    if *LogLevelConf::get() <= LogLevel::Error as i32 {
        eprintln!("{} {}", "[ERROR]".red(), s);
    }
}

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => ($crate::util::logger::__err(&format!($($arg)*)))
}

pub fn set_log_level(level: LogLevel) {
    let l = LogLevelConf::get();
    *l = level as i32;
}
