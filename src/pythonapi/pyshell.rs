use pyo3::{exceptions::PyTypeError, pyclass, pymethods, PyResult};
use serde::Deserialize;

use crate::{term::shell::Shell, util::anybase::heap_raw};

use super::shell_like::{PyTty, PyTtyWrapper, TtyType};

#[derive(Deserialize)]
pub struct PyTtyShellConf {
    pub shell: Option<String>,
}


pub fn handle_shell(inner: &mut Option<PyTtyWrapper>, shell_conf: PyTtyShellConf) -> PyResult<()> {
    let shell = shell_conf.shell.as_deref();
    let shell = Shell::build(shell);
    if let Err(e) = shell {
        return Err(PyTypeError::new_err(e.to_string()));
    }
    let shell = shell.unwrap();
    if inner.is_some() {
        return Err(PyTypeError::new_err(
            "Seems you defined more than one unwrappable object",
        ));
    }
    let shell = Box::new(shell) as TtyType;
    *inner = Some(PyTtyWrapper {
        tty: heap_raw(shell),
    });
    Ok(())
}

#[pyclass(extends=PyTty, subclass)]
pub struct PyShell {}

#[pymethods]
impl PyShell {
    #[new]
    #[pyo3(signature = (shell=None))]
    fn py_new(shell: Option<&str>) -> PyResult<(Self, PyTty)> {
        let shell = Shell::build(shell);
        if let Err(e) = shell {
            return Err(PyTypeError::new_err(e.to_string()));
        }
        let shell = shell.unwrap();
        let shell = Box::new(shell) as TtyType;
        Ok((
            PyShell {},
            PyTty::build(
                PyTtyWrapper {
                    tty: heap_raw(shell),
                },
            ),
        ))
    }
}
