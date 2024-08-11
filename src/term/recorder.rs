use std::error::Error;

use crate::term::tty::Tty;

pub trait Recorder<T>: Tty
where
    T: Tty,
{
    fn begin(&mut self) -> Result<(), Box<dyn Error>>;
    fn end(&mut self) -> Result<Vec<u8>, Box<dyn Error>>;
    fn exit(self) -> T;
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

    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let res = self.inner.write(data);
        if let Err(e) = res {
            return Err(e);
        }

        return Ok(());
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

    fn end(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        if !self.begin {
            return Err(Box::<dyn Error>::from("Not started"));
        }

        self.begin = false;

        let logged = self.logged.clone();
        self.logged.clear();

        return Ok(logged);
    }

    fn exit(self) -> T {
        return self.inner;
    }
}
