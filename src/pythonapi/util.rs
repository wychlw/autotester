use pyo3::{exceptions::PyRuntimeError, pyfunction, PyResult};

use crate::{ui::main::AppUi, util::logger::LogLevel};

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
    let level = crate::util::logger::get_log_level();
    match level {
        LogLevel::Debug => "Debug".to_string(),
        LogLevel::Info => "Info".to_string(),
        LogLevel::Warn => "Warn".to_string(),
        LogLevel::Error => "Error".to_string(),
    }
}

#[pyfunction]
pub fn run_ui() -> PyResult<()> {
    AppUi::new().map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(())
}
