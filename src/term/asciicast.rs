use std::{error::Error, time::SystemTime};

use super::{recorder::Recorder, tty::Tty};

pub struct Asciicast<T>
where
    T: Tty,
{
    inner: T,
    logged: Vec<u8>,
    begin: bool,
    begin_time: SystemTime
}

impl<T> Asciicast<T>
where
    T: Tty,
{
    pub fn build(inner: T) -> Asciicast<T> {
        Asciicast {
            inner,
            logged: Vec::new(),
            begin: false,
            begin_time: SystemTime::now()
        }
    }
}

impl<T> Tty for Asciicast<T>
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
            let time = self.begin_time.elapsed().unwrap();
            let timestamp = time.as_micros();
            let timestamp = timestamp as f64 / 1000.0;
            let line = format!("{{\"timestamp\": {}, \"event\": \"o\", \"data\": \"{}\"}}\n", timestamp, String::from_utf8(data.clone()).unwrap());
            self.logged.extend(line.as_bytes());
        }

        return Ok(data);
    }

    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let res = self.inner.write(data);

        res
    }
}

impl<T> Recorder<T> for Asciicast<T>
where
    T: Tty,
{
    fn begin(&mut self) -> Result<(), Box<dyn Error>> {
        self.logged.clear();
        
        let time = SystemTime::now();
        let timestamp = time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let front_line = format!(
            "{{\"version\": 2, \"width\": 80, \"height\": 24, \"timestamp\": {}, \"env\": {{ \"SHELL\": \"/bin/bash\", \"TERM\": \"VT100\" }} }}\n",
            timestamp
        );
        self.logged.extend(front_line.as_bytes());
        self.begin_time = SystemTime::now();
        self.begin = true;
        return Ok(());
    }

    fn end(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        if !self.begin {
            return Err(Box::<dyn Error>::from("Recorder not started."));
        }
        self.begin = false;
        let logged = self.logged.clone();
        self.logged.clear();
        return Ok(logged);
    }

    fn exit(self) -> T {
        self.inner
    }
}
