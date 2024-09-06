use crate::singleton;

pub enum LogLevel {
    Debug = 0,
    Info = 10,
    Warn = 20,
    Error = 30,
}

singleton!(LogLevelConf, i32, LogLevel::Debug as i32);

pub fn __log(s: &str) {
    if LogLevelConf::get().to_owned() <= LogLevel::Info as i32 {
        println!("{}", s);
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => (crate::util::logger::__log(&format!($($arg)*)))
}

pub fn __warn(s: &str) {
    if LogLevelConf::get().to_owned() <= LogLevel::Warn as i32 {
        eprintln!("{}", s);
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => (crate::util::logger::__warn(&format!($($arg)*)))
}

pub fn __err(s: &str) {
    if LogLevelConf::get().to_owned() <= LogLevel::Error as i32 {
        eprintln!("{}", s);
    }
}

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => (crate::util::logger::__err(&format!($($arg)*)))
}

pub fn set_log_level(level: LogLevel) {
    let l = LogLevelConf::get();
    *l = level as i32;
}
