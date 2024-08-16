use std::{collections::HashMap, error::Error, time::SystemTime};

use asciicast::{Entry, EventType, Header};
use serde_json::to_string;

use super::{recorder::Recorder, tty::Tty};

pub struct Asciicast<T>
where
    T: Tty,
{
    inner: T,
    head: Header,
    logged: Vec<Entry>,
    begin: bool,
    begin_time: SystemTime,
}

impl<T> Asciicast<T>
where
    T: Tty,
{
    pub fn build(inner: T) -> Asciicast<T> {
        Asciicast {
            inner,
            head: Header {
                version: 2,
                width: 80,
                height: 24,
                timestamp: None,
                duration: None,
                idle_time_limit: None,
                command: None,
                title: None,
                env: None,
                // env: HashMap::from([
                //     ("SHELL".to_string(), "/bin/sh".to_string()),
                //     ("TERM".to_string(), "VT100".to_string()),
                // ]),
            },
            logged: Vec::new(),
            begin: false,
            begin_time: SystemTime::now(),
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

        if self.begin && !data.is_empty() {
            let time = self.begin_time.elapsed().unwrap();
            let timestamp = time.as_millis();
            let timestamp = timestamp as f64 / 1000.0;
            self.logged.push(Entry {
                time: timestamp,
                event_type: EventType::Output,
                event_data: String::from_utf8(data.clone()).unwrap(),
            });
        }

        return Ok(data);
    }
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let data = self.inner.read_line();
        if let Err(e) = data {
            return Err(e);
        }
        let data = data.unwrap();

        if self.begin && !data.is_empty() {
            let time = self.begin_time.elapsed().unwrap();
            let timestamp = time.as_millis();
            let timestamp = timestamp as f64 / 1000.0;
            self.logged.push(Entry {
                time: timestamp,
                event_type: EventType::Output,
                event_data: String::from_utf8(data.clone()).unwrap(),
            });
        }

        return Ok(data);
    }
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        if self.begin {
            let time = self.begin_time.elapsed().unwrap();
            let timestamp = time.as_millis();
            let timestamp = timestamp as f64 / 1000.0;
            self.logged.push(Entry {
                time: timestamp,
                event_type: EventType::Input,
                event_data: String::from_utf8(data.to_vec()).unwrap(),
            });
        }
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
        self.head = Header {
            version: 2,
            width: 80,
            height: 24,
            timestamp: None,
            duration: None,
            idle_time_limit: None,
            command: None,
            title: None,
            env: Some(HashMap::from([
                ("SHELL".to_string(), "/bin/sh".to_string()),
                ("TERM".to_string(), "VT100".to_string()),
            ])),
        };
        self.begin_time = SystemTime::now();
        self.begin = true;
        return Ok(());
    }

    fn end(&mut self) -> Result<String, Box<dyn Error>> {
        if !self.begin {
            return Err(Box::<dyn Error>::from("Recorder not started."));
        }
        self.begin = false;
        let mut logged = String::new();
        let head = to_string(&self.head).unwrap();
        logged += &head;
        logged += "\n";
        for entry in &self.logged {
            let line = to_string(entry).unwrap();
            let line = line.replace("\\n", "\\r\\n"); // fix line ending
            logged += &line;
            logged += "\n";
        }
        logged += "\n";
        return Ok(logged);
    }

    fn exit(self) -> T {
        self.inner
    }
}
