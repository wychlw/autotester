//! Asciicast recorder. Multi thread version to log the output at the same time as the output goes into the terminal.
//!
//! The Asciicast recorder is a recorder that records the terminal output
//! in the asciicast v2 format.
//! This version may have problems with inner_mut/ref method.

use std::{
    collections::HashMap, error::Error, mem::replace, sync::{Arc, Mutex}, thread::{sleep, spawn, JoinHandle}, time::{Duration, SystemTime}
};

use asciicast::{Entry, EventType, Header};
use serde_json::to_string;

use crate::{consts::DURATION, impl_any, info};

use super::{
    recorder::Recorder,
    tty::{DummyTty, DynTty, Tty, WrapperTty},
};

pub struct Asciicast {
    inner: Arc<Mutex<DynTty>>,
    inner_took: Arc<Mutex<bool>>,
    head: Header,
    data: Arc<Mutex<Vec<u8>>>,
    logged: Arc<Mutex<Vec<Entry>>>,
    begin: Arc<Mutex<bool>>,
    begin_time: Arc<Mutex<SystemTime>>,
    thread: Option<JoinHandle<()>>,
}

impl Asciicast {
    pub fn build(inner: DynTty) -> Asciicast {
        let inner = Arc::new(Mutex::new(inner));

        let mut res = Asciicast {
            inner: inner.clone(),
            inner_took: Arc::new(Mutex::new(false)),
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
            data: Arc::new(Mutex::new(Vec::new())),
            logged: Arc::new(Mutex::new(Vec::new())),
            begin: Arc::new(Mutex::new(false)),
            begin_time: Arc::new(Mutex::new(SystemTime::now())),
            thread: None,
        };

        let inner = inner.clone();
        let inner_took = res.inner_took.clone();
        let data = res.data.clone();
        let logged = res.logged.clone();
        let begin = res.begin.clone();
        let begin_time = res.begin_time.clone();
        let process = move || loop {
            sleep(Duration::from_millis(DURATION));
            {
                let inner_took = inner_took.lock().unwrap();
                if *inner_took {
                    return;
                }
            }
            let new_data = {
                let mut inner = inner.lock().unwrap();
                let new_data = inner.read();
                if new_data.is_err() {
                    return;
                }
                new_data.unwrap()
            };

            {
                let begin = begin.lock().unwrap();
                if *begin && !new_data.is_empty() {
                    let time = begin_time.lock().unwrap().elapsed().unwrap();
                    let timestamp = time.as_millis();
                    let timestamp = timestamp as f64 / 1000.0;
                    let mut logged = logged.lock().unwrap();
                    logged.push(Entry {
                        time: timestamp,
                        event_type: EventType::Output,
                        event_data: String::from_utf8(new_data.clone()).unwrap_or_default(),
                    });
                }
            }

            {
                let mut data = data.lock().unwrap();
                data.extend(new_data);
            }
        };

        let thread = spawn(process);

        res.thread = Some(thread);

        info!("Create a Asciicast recorder to record.");

        res
    }
}

impl_any!(Asciicast);

impl Tty for Asciicast {
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let data = self.data.lock();
        if data.is_err() {
            return Err(Box::<dyn Error>::from("Read from Asciicast failed."));
        }
        let mut data = data.unwrap();
        let res = data.clone();
        data.clear();

        Ok(res)
    }
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut res = Vec::new();
        loop {
            sleep(Duration::from_millis(DURATION));
            let mut data = self.data.lock().unwrap();
            if data.is_empty() {
                continue;
            }
            res.push(data[0]);
            data.drain(0..1);
            if res.ends_with(&[0x0A]) {
                break;
            }
        }
        Ok(res)
    }
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        {
            let inner_took = self.inner_took.lock().unwrap();
            if *inner_took {
                return Err(Box::<dyn Error>::from("You've already exited."));
            }
        }
        {
            let inner = self.inner.clone();
            let mut inner = inner.lock().unwrap();
            inner.write(data)
        }
    }
}

impl WrapperTty for Asciicast {
    fn exit(mut self) -> DynTty {
        {
            let mut inner_took = self.inner_took.lock().unwrap();
            *inner_took = true;
        }
        let dummy = DummyTty {};
        self.swap(Box::new(dummy)).unwrap()
    }

    fn inner_mut(&mut self) -> &mut DynTty {
        panic!("Multi thread Asciicast recorder does not support inner_mut method... I have no idea how to implement it.");
    }

    fn inner_ref(&self) -> &DynTty {
        panic!("Multi thread Asciicast recorder does not support inner_mut method... I have no idea how to implement it.");
    }
}

impl Recorder for Asciicast {
    fn begin(&mut self) -> Result<(), Box<dyn Error>> {
        let logged = self.logged.lock();
        if logged.is_err() {
            return Err(Box::<dyn Error>::from("Recorder not started."));
        }
        let mut logged = logged.unwrap();
        logged.clear();

        let time = SystemTime::now();
        let begin_time = self.begin_time.lock();
        if begin_time.is_err() {
            return Err(Box::<dyn Error>::from("Recorder not started."));
        }
        let mut begin_time = begin_time.unwrap();
        *begin_time = time;

        let begin = self.begin.lock();
        if begin.is_err() {
            return Err(Box::<dyn Error>::from("Recorder not started."));
        }
        let mut begin = begin.unwrap();
        *begin = true;

        info!("Asciicast begin to record.");

        Ok(())
    }

    fn end(&mut self) -> Result<String, Box<dyn Error>> {
        let begin = self.begin.lock();
        if begin.is_err() {
            return Err(Box::<dyn Error>::from("Recorder not started."));
        }
        let mut begin = begin.unwrap();
        if !*begin {
            return Err(Box::<dyn Error>::from("Recorder not started."));
        }
        *begin = false;
        let mut res = String::new();
        let logged = self.logged.lock();
        if logged.is_err() {
            return Err(Box::<dyn Error>::from("Recorder not started."));
        }
        let logged = logged.unwrap();
        let head = to_string(&self.head).unwrap();
        res += &head;
        res += "\n";
        for entry in logged.iter() {
            let line = to_string(entry).unwrap();
            let line = line.replace("\\n", "\\r\\n");
            res += &line;
            res += "\n";
        }
        res += "\n";

        info!("Asciicast end recording...");

        Ok(res)
    }

    fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let begin = self.begin.lock();
        if let Err(e) = begin {
            return Err(Box::<dyn Error>::from(format!(
                "Recorder not started. Reason: {}",
                e
            )));
        }
        let mut begin = begin.unwrap();
        *begin = true;

        info!("Asciicast continue recording...");

        Ok(())
    }

    fn pause(&mut self) -> Result<(), Box<dyn Error>> {
        let begin = self.begin.lock();
        if let Err(e) = begin {
            return Err(Box::<dyn Error>::from(format!(
                "Recorder not started. Reason: {}",
                e
            )));
        }
        let mut begin = begin.unwrap();
        *begin = false;

        info!("Asciicast pause recording...");

        Ok(())
    }

    fn swap(&mut self, target: DynTty) -> Result<DynTty, Box<dyn Error>> {
        sleep(Duration::from_micros(DURATION));
        {
            let inner_took = self.inner_took.lock().unwrap();
            if *inner_took {
                return Err(Box::<dyn Error>::from("You've already exited."));
            }
        }
        let inner = self.inner.clone();
        let mut inner = inner.lock().unwrap();
        let res = replace(&mut *inner, target);
        Ok(res)
    }
}
