use std::ptr::null_mut;

use pyo3::{exceptions::PyTypeError, prelude::*};
use serde::Deserialize;

use crate::{
    logger::log,
    term::{
        asciicast::Asciicast,
        recorder::{Recorder, SimpleRecorder},
        serial::Serial,
        shell::Shell,
        ssh::Ssh,
        tty::{DynTty, WrapperTty},
    }, util::anybase::heap_raw,
};

pub type TtyType = DynTty;

pub struct PyTtyWrapper {
    pub tty: *mut TtyType,
}

impl PyTtyWrapper {
    pub fn take(&mut self) -> PyResult<*mut TtyType> {
        if self.tty.is_null() {
            return Err(PyTypeError::new_err(
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
            return Err(PyTypeError::new_err(
                "You gave me it, you will never own it again.",
            ));
        }
        Ok(unsafe { &*self.tty })
    }
    pub fn get_mut(&self) -> PyResult<&mut TtyType> {
        if self.tty.is_null() {
            return Err(PyTypeError::new_err(
                "You gave me it, you will never own it again.",
            ));
        }
        Ok(unsafe { &mut *self.tty })
    }
}

unsafe impl Send for PyTtyWrapper {}

#[pyclass(subclass)]
pub struct PyTty {
    inner: PyTtyWrapper,
}

impl PyTty {
    pub fn build(inner: PyTtyWrapper) -> Self {
        PyTty {
            inner
        }
    }
}

#[derive(Deserialize)]
struct PyTtyConf {
    // unwrapable
    wrap: Option<bool>,
    shell: Option<PyTtyShellConf>,

    // wrapable
    simple_recorder: Option<bool>,
    asciicast: Option<bool>,
}

#[derive(Deserialize)]
struct PyTtyShellConf {
    shell: Option<String>,
}

fn handel_wrap(inner: &mut Option<PyTtyWrapper>, be_wrapped: Option<&mut PyTty>) -> PyResult<()> {
    if be_wrapped.is_none() {
        return Err(PyTypeError::new_err(
            "be_wrapped must be provided when wrap is true",
        ));
    }
    let be_wrapped = be_wrapped.unwrap();

    *inner = Some(PyTtyWrapper {
        tty: be_wrapped.inner.take()?,
    });
    Ok(())
}

fn handel_shell(inner: &mut Option<PyTtyWrapper>, shell_conf: PyTtyShellConf) -> PyResult<()> {
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
fn handel_simple_recorder(inner: &mut Option<PyTtyWrapper>) -> PyResult<()> {
    if inner.is_none() {
        return Err(PyTypeError::new_err(
            "You must define at least one valid object",
        ));
    }
    let mut be_wrapped = inner.take().unwrap();
    if be_wrapped.tty.is_null() {
        return Err(PyTypeError::new_err(
            "You gave me it, you will never own it again.",
        ));
    }
    let tty = be_wrapped.safe_take()?;
    let tty = Box::into_inner(tty);
    let recorder = Box::new(SimpleRecorder::build(tty));
    let recorder = recorder as TtyType;
    *inner = Some(PyTtyWrapper {
        tty: heap_raw(recorder),
    });

    Ok(())
}
fn handel_asciicast(inner: &mut Option<PyTtyWrapper>) -> PyResult<()> {
    if inner.is_none() {
        return Err(PyTypeError::new_err(
            "You must define at least one valid object",
        ));
    }
    let mut be_wrapped = inner.take().unwrap();
    if be_wrapped.tty.is_null() {
        return Err(PyTypeError::new_err(
            "You gave me it, you will never own it again.",
        ));
    }
    let tty = be_wrapped.safe_take()?;
    let tty = Box::into_inner(tty);
    let recorder = Box::new(Asciicast::build(tty));
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
 */

#[pymethods]
impl PyTty {
    #[new]
    #[pyo3(signature = (conf, be_wrapped=None))]
    fn py_new(conf: &str, be_wrapped: Option<&mut PyTty>) -> PyResult<Self> {
        log(format!("Got conf: {}", conf));

        let conf: PyTtyConf = toml::from_str(conf).unwrap();

        let mut inner = None;

        if conf.wrap.is_some_and(|x| x) {
            handel_wrap(&mut inner, be_wrapped)?;
        }
        if let Some(shell_conf) = conf.shell {
            handel_shell(&mut inner, shell_conf)?;
        }
        if conf.simple_recorder.is_some_and(|x| x) {
            handel_simple_recorder(&mut inner)?;
        }
        if conf.asciicast.is_some_and(|x| x) {
            handel_asciicast(&mut inner)?;
        }

        if inner.is_none() {
            return Err(PyTypeError::new_err(
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
            .map_err(|e| PyTypeError::new_err(e.to_string()))
    }
    fn read_line(&mut self) -> PyResult<Vec<u8>> {
        let inner = self.inner.get_mut()?;
        (*inner)
            .read_line()
            .map_err(|e| PyTypeError::new_err(e.to_string()))
    }
    fn write(&mut self, data: &[u8]) -> PyResult<()> {
        let inner = self.inner.get_mut()?;
        (*inner)
            .write(data)
            .map_err(|e| PyTypeError::new_err(e.to_string()))
    }

    // WrapperTty begin

    fn exit(&mut self) -> PyResult<Self> {
        let inner = self.inner.safe_take()?;
        let inner = Box::into_inner(inner);
        let inner = inner.into_any();

        if let Some(_) = inner.downcast_ref::<Shell>() {
            Err(PyTypeError::new_err("Can't convert to the type you want"))
        } else if let Some(_) = inner.downcast_ref::<Serial>() {
            Err(PyTypeError::new_err("Can't convert to the type you want"))
        } else if let Some(_) = inner.downcast_ref::<Ssh>() {
            Err(PyTypeError::new_err("Can't convert to the type you want"))
        } else if let Some(_) = inner.downcast_ref::<SimpleRecorder>() {
            let inner = inner.downcast::<SimpleRecorder>().unwrap();
            let inner = inner.exit();
            Ok(PyTty {
                inner: PyTtyWrapper {
                    tty: heap_raw(inner),
                },
            })
        } else if let Some(_) = inner.downcast_ref::<Asciicast>() {
            let inner = inner.downcast::<Asciicast>().unwrap();
            let inner = inner.exit();
            Ok(PyTty {
                inner: PyTtyWrapper {
                    tty: heap_raw(inner),
                },
            })
        } else {
            Err(PyTypeError::new_err(
                "What type is this? How do you get it?",
            ))
        }
    }

    // Recorder begin

    fn begin(&mut self) -> PyResult<()> {
        let inner = self.inner.get_mut()?;
        let inner = inner.as_any_mut();

        if let Some(_) = inner.downcast_ref::<Shell>() {
            Err(PyTypeError::new_err("Can't convert to the type you want"))
        } else if let Some(_) = inner.downcast_ref::<Serial>() {
            Err(PyTypeError::new_err("Can't convert to the type you want"))
        } else if let Some(_) = inner.downcast_ref::<Ssh>() {
            Err(PyTypeError::new_err("Can't convert to the type you want"))
        } else if let Some(_) = inner.downcast_ref::<SimpleRecorder>() {
            let inner = inner.downcast_mut::<SimpleRecorder>().unwrap();
            inner
                .begin()
                .map_err(|e| PyTypeError::new_err(e.to_string()))
        } else if let Some(_) = inner.downcast_ref::<Asciicast>() {
            let inner = inner.downcast_mut::<Asciicast>().unwrap();
            inner
                .begin()
                .map_err(|e| PyTypeError::new_err(e.to_string()))
        } else {
            Err(PyTypeError::new_err(
                "What type is this? How do you get it?",
            ))
        }
    }

    fn end(&mut self) -> PyResult<String> {
        let inner = self.inner.get_mut()?;
        let inner = inner.as_any_mut();

        if let Some(_) = inner.downcast_ref::<Shell>() {
            Err(PyTypeError::new_err("Can't convert to the type you want"))
        } else if let Some(_) = inner.downcast_ref::<Serial>() {
            Err(PyTypeError::new_err("Can't convert to the type you want"))
        } else if let Some(_) = inner.downcast_ref::<Ssh>() {
            Err(PyTypeError::new_err("Can't convert to the type you want"))
        } else if let Some(_) = inner.downcast_ref::<SimpleRecorder>() {
            let inner = inner.downcast_mut::<SimpleRecorder>().unwrap();
            inner.end().map_err(|e| PyTypeError::new_err(e.to_string()))
        } else if let Some(_) = inner.downcast_ref::<Asciicast>() {
            let inner = inner.downcast_mut::<Asciicast>().unwrap();
            inner.end().map_err(|e| PyTypeError::new_err(e.to_string()))
        } else {
            Err(PyTypeError::new_err(
                "What type is this? How do you get it?",
            ))
        }
    }

    fn start(&mut self) -> PyResult<()> {
        let inner = self.inner.get_mut()?;
        let inner = inner.as_any_mut();

        if let Some(_) = inner.downcast_ref::<Shell>() {
            Err(PyTypeError::new_err("Can't convert to the type you want"))
        } else if let Some(_) = inner.downcast_ref::<Serial>() {
            Err(PyTypeError::new_err("Can't convert to the type you want"))
        } else if let Some(_) = inner.downcast_ref::<Ssh>() {
            Err(PyTypeError::new_err("Can't convert to the type you want"))
        } else if let Some(_) = inner.downcast_ref::<SimpleRecorder>() {
            let inner = inner.downcast_mut::<SimpleRecorder>().unwrap();
            inner
                .start()
                .map_err(|e| PyTypeError::new_err(e.to_string()))
        } else if let Some(_) = inner.downcast_ref::<Asciicast>() {
            let inner = inner.downcast_mut::<Asciicast>().unwrap();
            inner
                .start()
                .map_err(|e| PyTypeError::new_err(e.to_string()))
        } else {
            Err(PyTypeError::new_err(
                "What type is this? How do you get it?",
            ))
        }
    }

    fn pause(&mut self) -> PyResult<()> {
        let inner = self.inner.get_mut()?;
        let inner = inner.as_any_mut();

        if let Some(_) = inner.downcast_ref::<Shell>() {
            Err(PyTypeError::new_err("Can't convert to the type you want"))
        } else if let Some(_) = inner.downcast_ref::<Serial>() {
            Err(PyTypeError::new_err("Can't convert to the type you want"))
        } else if let Some(_) = inner.downcast_ref::<Ssh>() {
            Err(PyTypeError::new_err("Can't convert to the type you want"))
        } else if let Some(_) = inner.downcast_ref::<SimpleRecorder>() {
            let inner = inner.downcast_mut::<SimpleRecorder>().unwrap();
            inner
                .pause()
                .map_err(|e| PyTypeError::new_err(e.to_string()))
        } else if let Some(_) = inner.downcast_ref::<Asciicast>() {
            let inner = inner.downcast_mut::<Asciicast>().unwrap();
            inner
                .pause()
                .map_err(|e| PyTypeError::new_err(e.to_string()))
        } else {
            Err(PyTypeError::new_err(
                "What type is this? How do you get it?",
            ))
        }
    }

    fn swap(&mut self, other: &mut Self) -> PyResult<()> {
        let inner = self.inner.get_mut()?;
        let inner = inner.as_any_mut();

        if let Some(_) = inner.downcast_ref::<Shell>() {
            Err(PyTypeError::new_err("Can't convert to the type you want"))
        } else if let Some(_) = inner.downcast_ref::<Serial>() {
            Err(PyTypeError::new_err("Can't convert to the type you want"))
        } else if let Some(_) = inner.downcast_ref::<Ssh>() {
            Err(PyTypeError::new_err("Can't convert to the type you want"))
        } else if let Some(_) = inner.downcast_ref::<SimpleRecorder>() {
            let inner = inner.downcast_mut::<SimpleRecorder>().unwrap();
            let target = other.inner.safe_take()?;
            let target = Box::into_inner(target);
            let target = inner.swap(target);
            if let Err(e) = target {
                return Err(PyTypeError::new_err(e.to_string()));
            }
            let target = target.unwrap();
            other.inner.tty = heap_raw(target);
            Ok(())
        } else if let Some(_) = inner.downcast_ref::<Asciicast>() {
            let inner = inner.downcast_mut::<Asciicast>().unwrap();
            let target = other.inner.safe_take()?;
            let target = Box::into_inner(target);
            let target = inner.swap(target);
            if let Err(e) = target {
                return Err(PyTypeError::new_err(e.to_string()));
            }
            let target = target.unwrap();
            other.inner.tty = heap_raw(target);
            Ok(())
        } else {
            Err(PyTypeError::new_err(
                "What type is this? How do you get it?",
            ))
        }
    }
}

