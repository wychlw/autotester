use pyo3::{exceptions::PyRuntimeError, pyclass, pymethods, PyResult};
use serde::Deserialize;

use crate::util::anybase::heap_raw;

use super::shell_like::{handle_wrap, py_tty_inner, PyTty, PyTtyInner, TtyType};

#[derive(Deserialize)]
pub struct PyTeeConf {
    pub path: String,
}

pub fn handle_tee(inner: &mut Option<PyTtyInner>, tee_conf: PyTeeConf) -> PyResult<()> {
    let path = tee_conf.path;
    if inner.is_none() {
        return Err(PyRuntimeError::new_err(
            "You must define at least one valid object",
        ));
    }
    let mut be_wrapped = inner.take().unwrap();
    let be_wrapped = be_wrapped.safe_take()?;
    let be_wrapped = Box::into_inner(be_wrapped);
    let tee = Box::new(crate::cli::tee::Tee::build(be_wrapped, &path));
    let tee = tee as TtyType;
    *inner = Some(py_tty_inner(heap_raw(tee)));
    Ok(())
}

#[pyclass(extends=PyTty, subclass)]
pub struct Tee {}

#[pymethods]
impl Tee {
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

        Ok((Tee {}, PyTty::build(inner.unwrap())))
    }
}
