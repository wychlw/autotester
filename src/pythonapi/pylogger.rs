use pyo3::pyfunction;

use crate::{err, info, log};

#[pyfunction]
pub fn log(msg: &str) {
    log!("{}", msg);
}

#[pyfunction]
pub fn info(msg: &str) {
    info!("{}", msg);
}

#[pyfunction]
pub fn warn(msg: &str) {
    crate::warn!("{}", msg);
}

#[pyfunction]
pub fn err(msg: &str) {
    err!("{}", msg);
}
