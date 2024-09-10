use pyo3::{exceptions::PyTypeError, pyclass, pymethods, PyResult};
use serde::Deserialize;

use crate::{term::tee::Tee, util::anybase::heap_raw};

use super::shell_like::{handle_wrap, PyTty, PyTtyWrapper, TtyType};

#[derive(Deserialize)]
pub struct PyTeeConf {
    pub path: String,
}

pub fn handle_tee(inner: &mut Option<PyTtyWrapper>, tee_conf: PyTeeConf) -> PyResult<()> {
    let path = tee_conf.path;
    if inner.is_none() {
        return Err(PyTypeError::new_err(
            "You must define at least one valid object",
        ));
    }
    let mut be_wrapped = inner.take().unwrap();
    let be_wrapped = be_wrapped.safe_take()?;
    let be_wrapped = Box::into_inner(be_wrapped);
    let tee = Box::new(Tee::build(be_wrapped, &path));
    let tee = tee as TtyType;
    *inner = Some(PyTtyWrapper { tty: heap_raw(tee) });
    Ok(())
}

#[pyclass(extends=PyTty, subclass)]
pub struct PyTee {}

#[pymethods]
impl PyTee {
    #[new]
    fn py_new(be_wrapped: &mut PyTty, path: &str) -> PyResult<(Self, PyTty)> {
        let mut inner = None;

        handle_wrap(&mut inner, Some(be_wrapped))?;
        handle_tee(
            &mut inner,
            PyTeeConf {
                path: path.to_owned(),
            },
        )?;

        Ok((PyTee {}, PyTty::build(inner.unwrap())))
    }
}