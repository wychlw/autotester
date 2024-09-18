use pyo3::{exceptions::PyRuntimeError, pyclass, pymethods, PyResult};

use crate::util::anybase::heap_raw;

use super::shell_like::{handle_wrap, PyTty, PyTtyWrapper, TtyType};

pub fn handle_asciicast(inner: &mut Option<PyTtyWrapper>) -> PyResult<()> {
    if inner.is_none() {
        return Err(PyRuntimeError::new_err(
            "You must define at least one valid object",
        ));
    }
    let mut be_wrapped = inner.take().unwrap();
    let be_wrapped = be_wrapped.safe_take()?;
    let be_wrapped = Box::into_inner(be_wrapped);
    let tee = Box::new(crate::cli::asciicast::Asciicast::build(be_wrapped));
    let tee = tee as TtyType;
    *inner = Some(PyTtyWrapper { tty: heap_raw(tee) });
    Ok(())
}

#[pyclass(extends=PyTty, subclass)]
pub struct Asciicast {}

#[pymethods]
impl Asciicast {
    #[new]
    fn py_new(be_wrapped: &mut PyTty) -> PyResult<(Self, PyTty)> {
        let mut inner = None;

        handle_wrap(&mut inner, Some(be_wrapped))?;
        handle_asciicast(&mut inner)?;

        Ok((Asciicast {}, PyTty::build(inner.unwrap())))
    }
}
