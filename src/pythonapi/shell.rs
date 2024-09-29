use pyo3::{exceptions::PyRuntimeError, pyclass, pymethods, PyResult};
use serde::Deserialize;

use crate::util::anybase::heap_raw;

use super::shell_like::{py_tty_inner, PyTty, PyTtyInner, TtyType};

#[derive(Deserialize)]
pub struct ShellConf {
    pub shell: Option<String>,
}

pub fn handle_shell(inner: &mut Option<PyTtyInner>, shell_conf: ShellConf) -> PyResult<()> {
    let shell = shell_conf.shell.as_deref();
    let shell = crate::cli::shell::Shell::build(shell)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    if inner.is_some() {
        return Err(PyRuntimeError::new_err(
            "Seems you defined more than one unwrappable object",
        ));
    }
    let shell = Box::new(shell) as TtyType;
    *inner = Some(py_tty_inner(heap_raw(shell)));
    Ok(())
}

#[pyclass(extends=PyTty, subclass)]
pub struct Shell {}

#[pymethods]
impl Shell {
    #[new]
    #[pyo3(signature = (shell=None))]
    fn py_new(shell: Option<&str>) -> PyResult<(Self, PyTty)> {
        let shell = crate::cli::shell::Shell::build(shell)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        let shell = Box::new(shell) as TtyType;
        Ok((Shell {}, PyTty::build(py_tty_inner(heap_raw(shell)))))
    }
}
