//! Asciicast recorder. Single thread version to bypass inner_ref/mut problem.
//!
//! The Asciicast recorder is a recorder that records the terminal output
//! in the asciicast v2 format.
//! This version may not log the output at the same time as the output goes into the terminal.


use std::{collections::HashMap, error::Error, mem::replace, time::SystemTime};

use asciicast::{Entry, EventType, Header};
use serde_json::to_string;

use crate::{impl_any, info};

use super::{
    recorder::Recorder,
    tty::{DynTty, Tty, WrapperTty},
};

pub struct Asciicast {
    inner: DynTty,
    logged: Vec<Entry>,
    begin: bool,
    begin_time: SystemTime,
    head: Header,
}

impl Asciicast {
    pub fn build(inner: DynTty) -> Asciicast {
        Asciicast {
            inner,
            logged: Vec::new(),
            begin: false,
            begin_time: SystemTime::now(),
            head: Header {
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
            },
        }
    }
}

impl_any!(Asciicast);

impl Tty for Asciicast {
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let data = self.inner.read()?;

        if self.begin {
            let time = self.begin_time.elapsed().unwrap();
            let timestamp = time.as_millis();
            let timestamp = timestamp as f64 / 1000.0;
            self.logged.push(Entry {
                time: timestamp,
                event_type: EventType::Output,
                event_data: String::from_utf8(data.clone()).unwrap_or_default(),
            })
        }

        Ok(data)
    }
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let data = self.inner.read_line()?;

        if self.begin {
            let time = self.begin_time.elapsed().unwrap();
            let timestamp = time.as_millis();
            let timestamp = timestamp as f64 / 1000.0;
            self.logged.push(Entry {
                time: timestamp,
                event_type: EventType::Output,
                event_data: String::from_utf8(data.clone()).unwrap_or_default(),
            })
        }

        Ok(data)
    }
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        self.inner.write(data)?;

        Ok(())
    }
}

impl WrapperTty for Asciicast {
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

impl Recorder for Asciicast {
    fn begin(&mut self) -> Result<(), Box<dyn Error>> {
        self.logged.clear();
        self.begin = true;
        self.begin_time = SystemTime::now();

        Ok(())
    }

    fn end(&mut self) -> Result<String, Box<dyn Error>> {
        if !self.begin {
            return Err(Box::<dyn Error>::from("Not started"));
        }

        self.begin = false;

        let logged = self.logged.clone();
        self.logged.clear();
        let head = to_string(&self.head).unwrap();
        let mut res = String::new();
        res += &head;
        res += "\n";
        for entry in logged.iter() {
            let line = to_string(entry).unwrap();
            let line = line.replace("\\n", "\\r\\n");
            res += &line;
            res += "\n";
        }
        res += "\n";

        Ok(res)
    }
    fn pause(&mut self) -> Result<(), Box<dyn Error>> {
        self.begin = false;
        info!("Asciicast pause for recording...");
        Ok(())
    }
    fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.begin = true;
        info!("Asciicast continue for recording...");
        Ok(())
    }
    fn swap(&mut self, target: DynTty) -> Result<DynTty, Box<dyn Error>> {
        let inner = replace(&mut self.inner, target);
        Ok(inner)
    }
}
