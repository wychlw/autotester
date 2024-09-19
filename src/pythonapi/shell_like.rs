use std::ptr::null_mut;

use pyo3::{exceptions::PyRuntimeError, prelude::*};
use serde::Deserialize;

use crate::{
    cli::{
        asciicast::Asciicast,
        deansi::DeANSI,
        recorder::{Recorder, SimpleRecorder},
        tee::Tee,
        tty::{DynTty, WrapperTty},
    },
    exec::cli_exec::{CliTester, SudoCliTester},
    log,
    pythonapi::{asciicast::handle_asciicast, tee::handle_tee},
    util::anybase::heap_raw,
};

use super::{
    exec::handle_clitester,
    hook::TtyHook,
    shell::{handle_shell, ShellConf},
    tee::PyTeeConf,
};

pub type TtyType = DynTty;

pub struct PyTtyWrapper {
    pub tty: *mut TtyType,
}

impl PyTtyWrapper {
    pub fn take(&mut self) -> PyResult<*mut TtyType> {
        if self.tty.is_null() {
            return Err(PyRuntimeError::new_err(
                "You gave me it, you will never own it again.",
            ));
        }
        let res = self.tty;
        self.tty = null_mut();
        Ok(res)
    }
    pub fn safe_take(&mut self) -> PyResult<Box<TtyType>> {
        let res = self.take()?;
        Ok(unsafe { Box::from_raw(res) })
    }
    pub fn get(&self) -> PyResult<&TtyType> {
        if self.tty.is_null() {
            return Err(PyRuntimeError::new_err(
                "You gave me it, you will never own it again.",
            ));
        }
        Ok(unsafe { &*self.tty })
    }
    pub fn get_mut(&self) -> PyResult<&mut TtyType> {
        if self.tty.is_null() {
            return Err(PyRuntimeError::new_err(
                "You gave me it, you will never own it again.",
            ));
        }
        Ok(unsafe { &mut *self.tty })
    }
}

unsafe impl Send for PyTtyWrapper {}

#[pyclass(subclass)]
pub struct PyTty {
    pub inner: PyTtyWrapper,
}

impl PyTty {
    pub fn build(inner: PyTtyWrapper) -> Self {
        PyTty { inner }
    }
}

#[derive(Deserialize)]
struct PyTtyConf {
    // unwrapable
    wrap: Option<bool>,
    shell: Option<ShellConf>,
    tee: Option<PyTeeConf>,

    // wrapable
    simple_recorder: Option<bool>,
    asciicast: Option<bool>,

    exec: Option<PyTtyExecConf>,
}

#[derive(Deserialize)]
struct PyTtyExecConf {
    sudo: Option<bool>,
}

pub fn handle_wrap(
    inner: &mut Option<PyTtyWrapper>,
    be_wrapped: Option<&mut PyTty>,
) -> PyResult<()> {
    if be_wrapped.is_none() {
        return Err(PyRuntimeError::new_err(
            "be_wrapped must be provided when wrap is true",
        ));
    }
    let be_wrapped = be_wrapped.unwrap();

    *inner = Some(PyTtyWrapper {
        tty: be_wrapped.inner.take()?,
    });
    Ok(())
}

pub fn handle_simple_recorder(inner: &mut Option<PyTtyWrapper>) -> PyResult<()> {
    if inner.is_none() {
        return Err(PyRuntimeError::new_err(
            "You must define at least one valid object",
        ));
    }
    let mut be_wrapped = inner.take().unwrap();
    let tty = be_wrapped.safe_take()?;
    let tty = Box::into_inner(tty);
    let recorder = Box::new(SimpleRecorder::build(tty));
    let recorder = recorder as TtyType;
    *inner = Some(PyTtyWrapper {
        tty: heap_raw(recorder),
    });

    Ok(())
}

/**
 * Shell
 * Serial
 * Ssh
 * SimpleRecorder
 * Asciicast
 * TtyHook
 */

#[pymethods]
impl PyTty {
    #[new]
    #[pyo3(signature = (conf, be_wrapped=None))]
    /**
     * NOTICE!
     * This API is only for supporting with t-autotest.
     * In future every initlizer will be defined in
     * separate class, while new Tty-like object will
     * not be added in this class.
     */
    fn py_new(conf: &str, be_wrapped: Option<&mut PyTty>) -> PyResult<Self> {
        log!("Got conf: {}", conf);

        let conf: PyTtyConf = toml::from_str(conf).unwrap();

        let mut inner = None;

        if conf.wrap.is_some_and(|x| x) {
            handle_wrap(&mut inner, be_wrapped)?;
        }
        if let Some(shell_conf) = conf.shell {
            handle_shell(&mut inner, shell_conf)?;
        }
        if let Some(tee_conf) = conf.tee {
            handle_tee(&mut inner, tee_conf)?;
        }
        if conf.simple_recorder.is_some_and(|x| x) {
            handle_simple_recorder(&mut inner)?;
        }
        if conf.asciicast.is_some_and(|x| x) {
            handle_asciicast(&mut inner)?;
        }
        if conf.exec.is_some() {
            let exec_conf = conf.exec.unwrap();
            handle_clitester(&mut inner, exec_conf.sudo)?;
        }

        if inner.is_none() {
            return Err(PyRuntimeError::new_err(
                "You must define at least one valid object",
            ));
        }
        let inner = inner.unwrap();

        Ok(PyTty { inner })
    }

    // Tty begin

    fn read(&mut self) -> PyResult<Vec<u8>> {
        let inner = self.inner.get_mut()?;
        (*inner)
            .read()
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    fn read_line(&mut self) -> PyResult<Vec<u8>> {
        let inner = self.inner.get_mut()?;
        (*inner)
            .read_line()
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    fn write(&mut self, data: &[u8]) -> PyResult<()> {
        let inner = self.inner.get_mut()?;
        (*inner)
            .write(data)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }

    // WrapperTty begin

    fn exit(&mut self) -> PyResult<Self> {
        let inner = self.inner.safe_take()?;
        let inner = Box::into_inner(inner);
        let inner = inner.into_any();

        if inner.downcast_ref::<SimpleRecorder>().is_some() {
            let inner = inner.downcast::<SimpleRecorder>().unwrap();
            let inner = inner.exit();
            Ok(PyTty {
                inner: PyTtyWrapper {
                    tty: heap_raw(inner),
                },
            })
        } else if inner.downcast_ref::<Asciicast>().is_some() {
            let inner = inner.downcast::<Asciicast>().unwrap();
            let inner = inner.exit();
            Ok(PyTty {
                inner: PyTtyWrapper {
                    tty: heap_raw(inner),
                },
            })
        } else if inner.downcast_ref::<DeANSI>().is_some() {
            let inner = inner.downcast::<DeANSI>().unwrap();
            let inner = inner.exit();
            Ok(PyTty {
                inner: PyTtyWrapper {
                    tty: heap_raw(inner),
                },
            })
        } else if inner.downcast_ref::<Tee>().is_some() {
            let inner = inner.downcast::<Tee>().unwrap();
            let inner = inner.exit();
            Ok(PyTty {
                inner: PyTtyWrapper {
                    tty: heap_raw(inner),
                },
            })
        } else if inner.downcast_ref::<CliTester>().is_some() {
            let inner = inner.downcast::<CliTester>().unwrap();
            let inner = inner.exit();
            Ok(PyTty {
                inner: PyTtyWrapper {
                    tty: heap_raw(inner),
                },
            })
        } else if inner.downcast_ref::<SudoCliTester>().is_some() {
            let inner = inner.downcast::<SudoCliTester>().unwrap();
            let inner = inner.exit();
            Ok(PyTty {
                inner: PyTtyWrapper {
                    tty: heap_raw(inner),
                },
            })
        } else {
            Err(PyRuntimeError::new_err(
                "This type doesn't have function exit",
            ))
        }
    }

    // Recorder begin

    fn begin(&mut self) -> PyResult<()> {
        let inner = self.inner.get_mut()?;
        let inner = inner.as_any_mut();

        if inner.downcast_ref::<SimpleRecorder>().is_some() {
            let inner = inner.downcast_mut::<SimpleRecorder>().unwrap();
            inner
                .begin()
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))
        } else if inner.downcast_ref::<Asciicast>().is_some() {
            let inner = inner.downcast_mut::<Asciicast>().unwrap();
            inner
                .begin()
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))
        } else {
            Err(PyRuntimeError::new_err(
                "This type doesn't have function begin",
            ))
        }
    }

    fn end(&mut self) -> PyResult<String> {
        let inner = self.inner.get_mut()?;
        let inner = inner.as_any_mut();

        if inner.downcast_ref::<SimpleRecorder>().is_some() {
            let inner = inner.downcast_mut::<SimpleRecorder>().unwrap();
            inner
                .end()
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))
        } else if inner.downcast_ref::<Asciicast>().is_some() {
            let inner = inner.downcast_mut::<Asciicast>().unwrap();
            inner
                .end()
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))
        } else {
            Err(PyRuntimeError::new_err(
                "This type doesn't have function end",
            ))
        }
    }

    fn start(&mut self) -> PyResult<()> {
        let inner = self.inner.get_mut()?;
        let inner = inner.as_any_mut();

        if inner.downcast_ref::<SimpleRecorder>().is_some() {
            let inner = inner.downcast_mut::<SimpleRecorder>().unwrap();
            inner
                .start()
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))
        } else if inner.downcast_ref::<Asciicast>().is_some() {
            let inner = inner.downcast_mut::<Asciicast>().unwrap();
            inner
                .start()
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))
        } else {
            Err(PyRuntimeError::new_err(
                "This type doesn't have function start",
            ))
        }
    }

    fn pause(&mut self) -> PyResult<()> {
        let inner = self.inner.get_mut()?;
        let inner = inner.as_any_mut();

        if inner.downcast_ref::<SimpleRecorder>().is_some() {
            let inner = inner.downcast_mut::<SimpleRecorder>().unwrap();
            inner
                .pause()
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))
        } else if inner.downcast_ref::<Asciicast>().is_some() {
            let inner = inner.downcast_mut::<Asciicast>().unwrap();
            inner
                .pause()
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))
        } else {
            Err(PyRuntimeError::new_err(
                "This type doesn't have function pause",
            ))
        }
    }

    fn swap(&mut self, other: &mut Self) -> PyResult<()> {
        let inner = self.inner.get_mut()?;
        let inner = inner.as_any_mut();

        if inner.downcast_ref::<SimpleRecorder>().is_some() {
            let inner = inner.downcast_mut::<SimpleRecorder>().unwrap();
            let target = other.inner.safe_take()?;
            let target = Box::into_inner(target);
            let target = inner.swap(target);
            if let Err(e) = target {
                return Err(PyRuntimeError::new_err(e.to_string()));
            }
            let target = target.unwrap();
            other.inner.tty = heap_raw(target);
            Ok(())
        } else if inner.downcast_ref::<Asciicast>().is_some() {
            let inner = inner.downcast_mut::<Asciicast>().unwrap();
            let target = other.inner.safe_take()?;
            let target = Box::into_inner(target);
            let target = inner.swap(target);
            if let Err(e) = target {
                return Err(PyRuntimeError::new_err(e.to_string()));
            }
            let target = target.unwrap();
            other.inner.tty = heap_raw(target);
            Ok(())
        } else {
            Err(PyRuntimeError::new_err(
                "This type doesn't have function swap",
            ))
        }
    }

    // special for py tty hook to unhook
    fn unhook(&mut self) -> PyResult<Py<PyAny>> {
        let inner = self.inner.safe_take()?;
        let inner = Box::into_inner(inner);
        let inner = inner.into_any();

        if inner.downcast_ref::<TtyHook>().is_some() {
            let inner = inner.downcast::<TtyHook>().unwrap();
            let res = inner.inner;
            Ok(res)
        } else {
            Err(PyRuntimeError::new_err("You can only unhook Hook"))
        }
    }
}
