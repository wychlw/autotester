use std::error::Error;

use pyo3::{
    pyfunction,
    types::{PyByteArray, PyByteArrayMethods},
    Py, PyAny, Python,
};

use crate::{cli::tty::Tty, impl_any, util::anybase::heap_raw};

use super::shell_like::{py_tty_inner, PyTty};

#[pyfunction]
pub fn build_ttyhook(inner: Py<PyAny>) -> PyTty {
    let inner = TtyHook::build(inner);
    let inner = Box::new(inner);
    let inner = py_tty_inner(heap_raw(inner));
    PyTty { inner }
}

pub struct TtyHook {
    pub inner: Py<PyAny>,
}

impl TtyHook {
    pub fn build(inner: Py<PyAny>) -> Self {
        Self { inner }
    }
}

impl_any!(TtyHook);

impl Tty for TtyHook {
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        Python::with_gil(|py| {
            let res = self.inner.call_method0(py, "read")?;
            let res = res.bind(py);
            let res = PyByteArray::from_bound(res)?;
            let res = res.to_vec();
            Ok(res)
        })
    }
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        Python::with_gil(|py| {
            let res = self.inner.call_method0(py, "read_line")?;
            let res = res.bind(py);
            let res = PyByteArray::from_bound(res)?;
            let res = res.to_vec();
            Ok(res)
        })
    }
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        Python::with_gil(|py| {
            let data = PyByteArray::new_bound(py, data);
            self.inner.call_method1(py, "write", (data,))?;
            Ok(())
        })
    }
}
