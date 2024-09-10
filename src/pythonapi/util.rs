use pyo3::pyfunction;

use crate::util::logger::LogLevel;


#[pyfunction]
pub fn set_log_level(level: &str) {
    let level = match level {
        "Debug" => LogLevel::Debug,
        "Info" => LogLevel::Info,
        "Warn" => LogLevel::Warn,
        "Error" => LogLevel::Error,
        _ => LogLevel::Debug,
    };
    crate::util::logger::set_log_level(level);
}

#[pyfunction]
pub fn get_log_level() -> String {
    let level = crate::util::logger::LogLevelConf::get();
    match level {
        0 => "Debug".to_string(),
        10 => "Info".to_string(),
        20 => "Warn".to_string(),
        30 => "Error".to_string(),
        _ => "Debug".to_string(),
    }
}