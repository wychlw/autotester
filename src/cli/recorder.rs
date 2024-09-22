use std::{any::Any, error::Error, mem::replace};

use crate::{cli::tty::Tty, info, util::anybase::AnyBase};

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
        info!("Create a simple recorder to record.");
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
        let data = self.inner.read()?;

        if self.begin {
            self.logged.extend(data.clone());
        }

        Ok(data)
    }
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let data = self.inner.read_line()?;

        if self.begin {
            self.logged.extend(data.clone());
        }

        Ok(data)
    }
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        self.inner.write(data)?;

        Ok(())
    }
}

impl WrapperTty for SimpleRecorder {
    fn exit(self) -> DynTty {
        self.inner
    }

    fn inner_ref(&self) -> &DynTty {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut DynTty {
        &mut self.inner
    }
}

impl Recorder for SimpleRecorder
{
    fn begin(&mut self) -> Result<(), Box<dyn Error>> {
        self.logged.clear();
        self.begin = true;

        info!("Recorder begin to record.");

        Ok(())
    }

    fn end(&mut self) -> Result<String, Box<dyn Error>> {
        if !self.begin {
            return Err(Box::<dyn Error>::from("Not started"));
        }

        self.begin = false;

        let logged = self.logged.clone();
        self.logged.clear();

        info!("Recorder end to record.");

        Ok(String::from_utf8(logged).unwrap())
    }
    fn pause(&mut self) -> Result<(), Box<dyn Error>> {
        self.begin = false;
        info!("Recorder pause for recording...");
        Ok(())
    }
    fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.begin = true;
        info!("Recorder continue for recording...");
        Ok(())
    }
    fn swap(&mut self, target: DynTty) -> Result<DynTty, Box<dyn Error>> {
        let inner = replace(&mut self.inner, target);
        Ok(inner)
    }
}
