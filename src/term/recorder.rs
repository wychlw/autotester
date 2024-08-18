use std::{error::Error, mem::replace};

use crate::term::tty::Tty;

use super::tty::WrapperTty;

pub trait Recorder<T>: WrapperTty<T>
where
    T: Tty,
{
    fn begin(&mut self) -> Result<(), Box<dyn Error>>;
    fn end(&mut self) -> Result<String, Box<dyn Error>>;

    /**
     * Swap the inner Tty object at runtime.
     */
    fn swap(&mut self, target: T) -> Result<T, Box<dyn Error>>;
}

pub struct SimpleRecorder<T>
where
    T: Tty,
{
    inner: T,
    logged: Vec<u8>,
    begin: bool,
}

impl<T> SimpleRecorder<T>
where
    T: Tty,
{
    pub fn build(inner: T) -> SimpleRecorder<T> {
        SimpleRecorder {
            inner,
            logged: Vec::new(),
            begin: false,
        }
    }
}

impl<T> Tty for SimpleRecorder<T>
where
    T: Tty,
{
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

        return Ok(());
    }
}

impl<T> WrapperTty<T> for SimpleRecorder<T>
where
    T: Tty,
{
    fn exit(self) -> T {
        self.inner
    }
}

impl<T> Recorder<T> for SimpleRecorder<T>
where
    T: Tty,
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
    fn swap(&mut self, target: T) -> Result<T, Box<dyn Error>> {
        let inner = replace(&mut self.inner, target);
        return Ok(inner);
    }
}
