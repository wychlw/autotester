use pyo3::{exceptions::PyTypeError, pyclass, pymethods, PyResult};

use crate::{term::shell::Shell, util::anybase::heap_raw};

use super::shell_like::{PyTty, PyTtyWrapper, TtyType};

pub fn handel_shell(inner: &mut Option<PyTtyWrapper>, shell_conf: Option<&str>) -> PyResult<()> {
    let shell = Shell::build(shell_conf);
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
