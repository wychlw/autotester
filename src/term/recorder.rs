use std::{any::Any, error::Error, mem::replace};

use crate::{consts::SHELL_PROMPT, term::tty::Tty, util::anybase::AnyBase};

use super::tty::{DynTty, WrapperTty};

pub trait Recorder: WrapperTty {
    fn begin(&mut self) -> Result<(), Box<dyn Error>>;
    fn end(&mut self) -> Result<String, Box<dyn Error>>;
    fn start(&mut self) -> Result<(), Box<dyn Error>>;
    fn pause(&mut self) -> Result<(), Box<dyn Error>>;

    /**
     * Swap the inner Tty object at runtime.
     */
    fn swap(&mut self, target: DynTty) -> Result<DynTty, Box<dyn Error>>;
}

pub struct SimpleRecorder {
    inner: DynTty,
    logged: Vec<u8>,
    begin: bool,
}

impl SimpleRecorder {
    pub fn build(inner: DynTty) -> SimpleRecorder {
        SimpleRecorder {
            inner,
            logged: Vec::new(),
            begin: false,
        }
    }
}

impl AnyBase for SimpleRecorder {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Tty for SimpleRecorder {
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let data = self.inner.read();
        if let Err(e) = data {
            return Err(e);
        }
        let data = data.unwrap();

        if self.begin {
            self.logged.extend(data.clone());
        }

        return Ok(data);
    }
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let data = self.inner.read_line();
        if let Err(e) = data {
            return Err(e);
        }
        let data = data.unwrap();

        if self.begin {
            self.logged.extend(data.clone());
        }

        return Ok(data);
    }
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let res = self.inner.write(data);
        if let Err(e) = res {
            return Err(e);
        }

        if self.begin {
            // For echo back:
            let line = SHELL_PROMPT.as_bytes();
            self.logged.extend(line);
            self.logged.extend(data);
        }

        return Ok(());
    }
}

impl WrapperTty for SimpleRecorder {
    fn exit(self) -> DynTty {
        self.inner
    }
}

impl Recorder for SimpleRecorder
{
    fn begin(&mut self) -> Result<(), Box<dyn Error>> {
        self.logged.clear();
        self.begin = true;
        return Ok(());
    }

    fn end(&mut self) -> Result<String, Box<dyn Error>> {
        if !self.begin {
            return Err(Box::<dyn Error>::from("Not started"));
        }

        self.begin = false;

        let logged = self.logged.clone();
        self.logged.clear();

        return Ok(String::from_utf8(logged).unwrap());
    }
    fn pause(&mut self) -> Result<(), Box<dyn Error>> {
        self.begin = false;
        return Ok(());
    }
    fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.begin = true;
        return Ok(());
    }
    fn swap(&mut self, target: DynTty) -> Result<DynTty, Box<dyn Error>> {
        let inner = replace(&mut self.inner, target);
        return Ok(inner);
    }
}
