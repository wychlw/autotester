use pyo3::{exceptions::PyRuntimeError, pyclass, pymethods, PyRefMut, PyResult};

use crate::{
    exec::{
        cli_api::{CliTestApi, SudoCliTestApi},
        cli_exec::CliTester,
        cli_exec_sudo::SudoCliTester,
    },
    util::anybase::heap_raw,
};

use super::shell_like::{handle_wrap, PyTty, PyTtyWrapper, TtyType};

pub fn handle_clitester(inner: &mut Option<PyTtyWrapper>, need_sudo: Option<bool>) -> PyResult<()> {
    if inner.is_none() {
        return Err(PyRuntimeError::new_err(
            "You must define at least one valid object",
        ));
    }
    let mut be_wrapped = inner.take().unwrap();
    if be_wrapped.tty.is_null() {
        return Err(PyRuntimeError::new_err(
            "You gave me it, you will never own it again.",
        ));
    }
    let tty = be_wrapped.safe_take()?;
    let tty = Box::into_inner(tty);
    let need_sudo = need_sudo.unwrap_or(true);
    let res = if need_sudo {
        let res = Box::new(SudoCliTester::build(tty));
        res as TtyType
    } else {
        let res = Box::new(CliTester::build(tty));
        res as TtyType
    };
    *inner = Some(PyTtyWrapper { tty: heap_raw(res) });
    Ok(())
}

#[pyclass(extends=PyTty, subclass)]
pub struct Exec {}

#[pymethods]
impl Exec {
    #[new]
    #[pyo3(signature = (be_wrapped, sudo=None))]
    fn py_new(be_wrapped: &mut PyTty, sudo: Option<bool>) -> PyResult<(Self, PyTty)> {
        let mut inner = None;

        handle_wrap(&mut inner, Some(be_wrapped))?;
        handle_clitester(&mut inner, sudo)?;

        Ok((Exec {}, PyTty::build(inner.unwrap())))
    }

    #[pyo3(signature = (script, timeout=None))]
    fn script_run(
        mut self_: PyRefMut<'_, Self>,
        script: &str,
        timeout: Option<u32>,
    ) -> PyResult<()> {
        let self_ = self_.as_mut();
        let inner = self_.inner.get_mut()?;
        let inner = inner.as_any_mut();

        let timeout = timeout.unwrap_or(30);

        if inner.downcast_ref::<CliTester>().is_some() {
            let inner = inner.downcast_mut::<CliTester>().unwrap();
            inner
                .script_run(script, timeout)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        } else if inner.downcast_ref::<SudoCliTester>().is_some() {
            let inner = inner.downcast_mut::<SudoCliTester>().unwrap();
            inner
                .script_run(script, timeout)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        } else {
            return Err(PyRuntimeError::new_err(
                "Can't find the right object to run the script",
            ));
        }
        Ok(())
    }

    #[pyo3(signature = (script, timeout=None))]
    fn assert_script_run(
        mut self_: PyRefMut<'_, Self>,
        script: &str,
        timeout: Option<u32>,
    ) -> PyResult<()> {
        let self_ = self_.as_mut();
        let inner = self_.inner.get_mut()?;
        let inner = inner.as_any_mut();

        let timeout = timeout.unwrap_or(30);

        if inner.downcast_ref::<CliTester>().is_some() {
            let inner = inner.downcast_mut::<CliTester>().unwrap();
            inner
                .assert_script_run(script, timeout)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        } else if inner.downcast_ref::<SudoCliTester>().is_some() {
            let inner = inner.downcast_mut::<SudoCliTester>().unwrap();
            inner
                .assert_script_run(script, timeout)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        } else {
            return Err(PyRuntimeError::new_err(
                "Can't find the right object to run the script",
            ));
        }
        Ok(())
    }

    fn background_script_run(mut self_: PyRefMut<'_, Self>, script: &str) -> PyResult<()> {
        let self_ = self_.as_mut();
        let inner = self_.inner.get_mut()?;
        let inner = inner.as_any_mut();

        if inner.downcast_ref::<CliTester>().is_some() {
            let inner = inner.downcast_mut::<CliTester>().unwrap();
            inner
                .background_script_run(script)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        } else if inner.downcast_ref::<SudoCliTester>().is_some() {
            let inner = inner.downcast_mut::<SudoCliTester>().unwrap();
            inner
                .background_script_run(script)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        } else {
            return Err(PyRuntimeError::new_err(
                "Can't find the right object to run the script",
            ));
        }
        Ok(())
    }

    fn writeln(mut self_: PyRefMut<'_, Self>, script: &str) -> PyResult<()> {
        let self_ = self_.as_mut();
        let inner = self_.inner.get_mut()?;
        let inner = inner.as_any_mut();

        if inner.downcast_ref::<CliTester>().is_some() {
            let inner = inner.downcast_mut::<CliTester>().unwrap();
            inner
                .writeln(script)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        } else if inner.downcast_ref::<SudoCliTester>().is_some() {
            let inner = inner.downcast_mut::<SudoCliTester>().unwrap();
            inner
                .writeln(script)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        } else {
            return Err(PyRuntimeError::new_err(
                "Can't find the right object to run the script",
            ));
        }
        Ok(())
    }

    #[pyo3(signature = (expected, timeout=None))]
    fn wait_serial(
        mut self_: PyRefMut<'_, Self>,
        expected: &str,
        timeout: Option<u32>,
    ) -> PyResult<()> {
        let self_ = self_.as_mut();
        let inner = self_.inner.get_mut()?;
        let inner = inner.as_any_mut();

        let timeout = timeout.unwrap_or(30);

        if inner.downcast_ref::<CliTester>().is_some() {
            let inner = inner.downcast_mut::<CliTester>().unwrap();
            inner
                .wait_serial(expected, timeout)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        } else if inner.downcast_ref::<SudoCliTester>().is_some() {
            let inner = inner.downcast_mut::<SudoCliTester>().unwrap();
            inner
                .wait_serial(expected, timeout)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        } else {
            return Err(PyRuntimeError::new_err(
                "Can't find the right object to run the script",
            ));
        }
        Ok(())
    }

    #[pyo3(signature = (script, timeout=None))]
    fn script_sudo(
        mut self_: PyRefMut<'_, Self>,
        script: &str,
        timeout: Option<u32>,
    ) -> PyResult<()> {
        let self_ = self_.as_mut();
        let inner = self_.inner.get_mut()?;
        let inner = inner.as_any_mut();

        let timeout = timeout.unwrap_or(30);

        if inner.downcast_ref::<SudoCliTester>().is_some() {
            let inner = inner.downcast_mut::<SudoCliTester>().unwrap();
            inner
                .script_sudo(script, timeout)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        } else {
            return Err(PyRuntimeError::new_err(
                "Can't find the right object to run the script",
            ));
        }
        Ok(())
    }

    #[pyo3(signature = (script, timeout=None))]
    fn assert_script_sudo(
        mut self_: PyRefMut<'_, Self>,
        script: &str,
        timeout: Option<u32>,
    ) -> PyResult<()> {
        let self_ = self_.as_mut();
        let inner = self_.inner.get_mut()?;
        let inner = inner.as_any_mut();

        let timeout = timeout.unwrap_or(30);

        if inner.downcast_ref::<SudoCliTester>().is_some() {
            let inner = inner.downcast_mut::<SudoCliTester>().unwrap();
            inner
                .assert_script_sudo(script, timeout)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        } else {
            return Err(PyRuntimeError::new_err(
                "Can't find the right object to run the script",
            ));
        }
        Ok(())
    }
}
