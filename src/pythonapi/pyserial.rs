use pyo3::{exceptions::PyRuntimeError, pyclass, pymethods, PyResult};

use crate::{term::serial::Serial, util::anybase::heap_raw};

use super::shell_like::{PyTty, PyTtyWrapper, TtyType};

#[pyclass(extends=PyTty, subclass)]
pub struct PySerial {}

#[pymethods]
impl PySerial {
    #[new]
    #[pyo3(signature = (port, baud))]
    fn py_new(port: &str, baud: u32) -> PyResult<(Self, PyTty)> {
        let serial = Serial::build(port, baud).map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        let serial = Box::new(serial) as TtyType;
        Ok((
            PySerial {},
            PyTty::build(PyTtyWrapper {
                tty: heap_raw(serial),
            }),
        ))
    }
}
