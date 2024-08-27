use std::{
    ops::Deref,
    ptr::{null, null_mut},
};

use pyo3::{exceptions::PyTypeError, prelude::*};
use serde::Deserialize;

use crate::{
    exec::cli_api::{CliTestApi, SudoCliTestApi},
    term::{
        recorder::{Recorder, SimpleRecorder},
        shell::Shell,
        tty::{DynTty, Tty, WrapperTty},
    },
    util::anybase::AnyBase,
};
enum TtyType {
    SelfWrapper(PyTtyWrapper),
    Tty(Box<dyn Tty>),
    WrapperTty(Box<dyn WrapperTty>),
    Recorder(Box<dyn Recorder>),
    CliTest(Box<dyn CliTestApi>),
    SudoCliTestApi(Box<dyn SudoCliTestApi>),
}

struct PyTtyWrapper {
    tty: *mut TtyType,
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

impl AnyBase for PyTtyWrapper {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

impl Tty for PyTtyWrapper {
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if self.tty.is_null() {
            return Err("You gave me it, you will never own it again.".into());
        }
        let tty = unsafe { &mut *self.tty };
        match tty {
            TtyType::SelfWrapper(tty) => tty.read(),
            TtyType::Tty(tty) => tty.read(),
            TtyType::WrapperTty(tty) => tty.read(),
            TtyType::Recorder(tty) => tty.read(),
            TtyType::CliTest(tty) => tty.read(),
            TtyType::SudoCliTestApi(tty) => tty.read(),
        }
    }
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if self.tty.is_null() {
            return Err("You gave me it, you will never own it again.".into());
        }
        let tty = unsafe { &mut *self.tty };
        match tty {
            TtyType::SelfWrapper(tty) => tty.read_line(),
            TtyType::Tty(tty) => tty.read_line(),
            TtyType::WrapperTty(tty) => tty.read_line(),
            TtyType::Recorder(tty) => tty.read_line(),
            TtyType::CliTest(tty) => tty.read_line(),
            TtyType::SudoCliTestApi(tty) => tty.read_line(),
        }
    }
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        if self.tty.is_null() {
            return Err("You gave me it, you will never own it again.".into());
        }
        let tty = unsafe { &mut *self.tty };
        match tty {
            TtyType::SelfWrapper(tty) => tty.write(data),
            TtyType::Tty(tty) => tty.write(data),
            TtyType::WrapperTty(tty) => tty.write(data),
            TtyType::Recorder(tty) => tty.write(data),
            TtyType::CliTest(tty) => tty.write(data),
            TtyType::SudoCliTestApi(tty) => tty.write(data),
        }
    }
}

impl WrapperTty for PyTtyWrapper {
    fn exit(mut self) -> DynTty {
        let tty = self.take().unwrap();
        let tty = unsafe { Box::from_raw(tty) };
        if let TtyType::SelfWrapper(tty) = *tty {
            return Box::from(tty);
        } else {
            panic!("Though I really want to do this. 
But now you are in python, which let rust don't know which type you really want to call, let it can't decide the size.
Simply, you can't exit from a non-SelfWrapper from python.
If you really want to wrap lots of Tty, you should do it with SelfWrapper.");
        }
    }
}

impl Recorder for PyTtyWrapper {
    fn begin(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.tty.is_null() {
            return Err("You gave me it, you will never own it again.".into());
        }
        let tty = unsafe { &mut *self.tty };
        match tty {
            TtyType::SelfWrapper(tty) => tty.begin(),
            TtyType::Tty(_) => Err("You can't begin a Tty".into()),
            TtyType::WrapperTty(_) => Err("You can't begin a WrapperTty".into()),
            TtyType::Recorder(tty) => tty.begin(),
            TtyType::CliTest(_) => Err("You can't begin a CliTest".into()),
            TtyType::SudoCliTestApi(_) => Err("You can't begin a SudoCliTestApi".into()),
        }
    }
    fn end(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        if self.tty.is_null() {
            return Err("You gave me it, you will never own it again.".into());
        }
        let tty = unsafe { &mut *self.tty };
        match tty {
            TtyType::SelfWrapper(tty) => tty.end(),
            TtyType::Tty(_) => Err("You can't end a Tty".into()),
            TtyType::WrapperTty(_) => Err("You can't end a WrapperTty".into()),
            TtyType::Recorder(tty) => tty.end(),
            TtyType::CliTest(_) => Err("You can't end a CliTest".into()),
            TtyType::SudoCliTestApi(_) => Err("You can't end a SudoCliTestApi".into()),
        }
    }
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.tty.is_null() {
            return Err("You gave me it, you will never own it again.".into());
        }
        let tty = unsafe { &mut *self.tty };
        match tty {
            TtyType::SelfWrapper(tty) => tty.start(),
            TtyType::Tty(_) => Err("You can't start a Tty".into()),
            TtyType::WrapperTty(_) => Err("You can't start a WrapperTty".into()),
            TtyType::Recorder(tty) => tty.start(),
            TtyType::CliTest(_) => Err("You can't start a CliTest".into()),
            TtyType::SudoCliTestApi(_) => Err("You can't start a SudoCliTestApi".into()),
        }
    }
    fn pause(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        todo!();
    }
    fn swap(&mut self, target: DynTty) -> Result<DynTty, Box<dyn std::error::Error>> {
        todo!();
    }
}

unsafe impl Send for PyTtyWrapper {}

#[pyclass]
struct PyTty {
    inner: PyTtyWrapper,
}

#[derive(Deserialize)]
struct PyTtyConf {
    // unwrapable
    wrap: Option<bool>,
    shell: Option<PyTtyShellConf>,

    // wrapable
    simple_recorder: Option<bool>,
}

#[derive(Deserialize)]
struct PyTtyShellConf {
    shell: Option<String>,
}

impl PyTty {
    fn handel_wrap(
        inner: &mut Option<PyTtyWrapper>,
        be_wrapped: Option<&mut PyTty>,
    ) -> PyResult<()> {
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
        *inner = Some(PyTtyWrapper {
            tty: Box::into_raw(Box::new(TtyType::Tty(Box::new(shell)))),
        });
        Ok(())
    }
    fn handel_simple_recorder(inner: &mut Option<PyTtyWrapper>) -> PyResult<()> {
        if inner.is_none() {
            return Err(PyTypeError::new_err(
                "You must define at least one valid object",
            ));
        }
        let be_wrapped = inner.take().unwrap();
        if be_wrapped.tty.is_null() {
            return Err(PyTypeError::new_err(
                "You gave me it, you will never own it again.",
            ));
        }
        let tty = Box::from(be_wrapped);
        let recorder = SimpleRecorder::build(tty);
        *inner = Some(PyTtyWrapper {
            tty: Box::into_raw(Box::new(TtyType::Recorder(Box::new(recorder)))),
        });
        Ok(())
    }
}

#[pymethods]
impl PyTty {
    #[new]
    #[pyo3(signature = (conf, be_wrapped=None))]
    fn py_new(conf: &str, be_wrapped: Option<&mut PyTty>) -> PyResult<Self> {
        let conf: PyTtyConf = serde_json::from_str(conf).unwrap();

        let mut inner = None;

        if conf.wrap.is_some_and(|x| x) {
            PyTty::handel_wrap(&mut inner, be_wrapped)?;
        }

        if let Some(shell_conf) = conf.shell {
            PyTty::handel_shell(&mut inner, shell_conf)?;
        }

        if conf.simple_recorder.is_some_and(|x| x) {
            PyTty::handel_simple_recorder(&mut inner)?;
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
        let res = match inner {
            TtyType::SelfWrapper(tty) => tty.read(),
            TtyType::Tty(tty) => tty.read(),
            TtyType::WrapperTty(tty) => tty.read(),
            TtyType::Recorder(tty) => tty.read(),
            TtyType::CliTest(tty) => tty.read(),
            TtyType::SudoCliTestApi(tty) => tty.read(),
        };
        res.map_err(|e| PyTypeError::new_err(e.to_string()))
    }
    fn read_line(&mut self) -> PyResult<Vec<u8>> {
        let inner = self.inner.get_mut()?;
        let res = match inner {
            TtyType::SelfWrapper(tty) => tty.read_line(),
            TtyType::Tty(tty) => tty.read_line(),
            TtyType::WrapperTty(tty) => tty.read_line(),
            TtyType::Recorder(tty) => tty.read_line(),
            TtyType::CliTest(tty) => tty.read_line(),
            TtyType::SudoCliTestApi(tty) => tty.read_line(),
        };
        res.map_err(|e| PyTypeError::new_err(e.to_string()))
    }
    fn write(&mut self, data: &[u8]) -> PyResult<()> {
        let inner = self.inner.get_mut()?;
        let res = match inner {
            TtyType::SelfWrapper(tty) => tty.write(data),
            TtyType::Tty(tty) => tty.write(data),
            TtyType::WrapperTty(tty) => tty.write(data),
            TtyType::Recorder(tty) => tty.write(data),
            TtyType::CliTest(tty) => tty.write(data),
            TtyType::SudoCliTestApi(tty) => tty.write(data),
        };
        res.map_err(|e| PyTypeError::new_err(e.to_string()))
    }

    // WrapperTty begin

    fn exit(&mut self) -> PyResult<Self> {
        let inner = self.inner.take()?;
        let inner = unsafe { Box::from_raw(inner) };
        let res = match *inner {
            TtyType::SelfWrapper(tty) => Ok(PyTty { inner: tty }),
            _ => Err(PyTypeError::new_err("Though I really want to do this. 
But now you are in python, which let rust don't know which type you really want to call, let it can't decide the size.
Simply, you can't exit from a non-SelfWrapper from python.
If you really want to wrap lots of Tty, you should do it with SelfWrapper.")),
        };

        res
    }
}
