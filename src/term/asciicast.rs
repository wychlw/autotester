use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, Mutex},
    thread::{sleep, spawn, JoinHandle},
    time::{Duration, SystemTime},
};

use asciicast::{Entry, EventType, Header};
use serde_json::to_string;

use crate::consts::DURATION;

use super::{recorder::Recorder, tty::Tty};

pub struct Asciicast<T>
where
    T: Tty,
{
    inner: Arc<Mutex<Option<T>>>,
    head: Header,
    data: Arc<Mutex<Vec<u8>>>,
    logged: Arc<Mutex<Vec<Entry>>>,
    begin: Arc<Mutex<bool>>,
    begin_time: Arc<Mutex<SystemTime>>,
    thread: Option<JoinHandle<()>>,
}

impl<T> Asciicast<T>
where
    T: Tty + Send + 'static,
{
    pub fn build(inner: T) -> Asciicast<T> {
        let inner = Arc::new(Mutex::new(Some(inner)));

        let mut res = Asciicast::<T> {
            inner: inner.clone(),
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
        let data = res.data.clone();
        let logged = res.logged.clone();
        let begin = res.begin.clone();
        let begin_time = res.begin_time.clone();
        let process = move || loop {
            sleep(Duration::from_millis(DURATION));
            let mut inner = inner.lock().unwrap();
            if inner.is_none() {
                return;
            }
            let inner = inner.as_mut().unwrap();
            let new_data = inner.read();
            if let Err(_) = new_data {
                return;
            }
            let new_data = new_data.unwrap();

            let begin = begin.lock().unwrap();
            if *begin && !new_data.is_empty() {
                let time = begin_time.lock().unwrap().elapsed().unwrap();
                let timestamp = time.as_millis();
                let timestamp = timestamp as f64 / 1000.0;
                let mut logged = logged.lock().unwrap();
                logged.push(Entry {
                    time: timestamp,
                    event_type: EventType::Output,
                    event_data: String::from_utf8(new_data.clone()).unwrap(),
                });
            }

            let mut data = data.lock().unwrap();
            data.extend(new_data);
        };

        let thread = spawn(process);

        res.thread = Some(thread);

        res
    }
}

impl<T> Tty for Asciicast<T>
where
    T: Tty,
{
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let data = self.data.lock();
        if let Err(_) = data {
            return Err(Box::<dyn Error>::from("Read from Asciicast failed."));
        }
        let mut data = data.unwrap();
        let res = data.clone();
        data.clear();

        return Ok(res);
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
        return Ok(res);
    }
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let begin = self.begin.lock();
        if let Err(_) = begin {
            return Err(Box::<dyn Error>::from("Recorder not started."));
        }
        let begin = begin.unwrap();
        if *begin {
            let begin_time = self.begin_time.lock().unwrap();
            let time = begin_time.elapsed().unwrap();
            let timestamp = time.as_millis();
            let timestamp = timestamp as f64 / 1000.0;
            let mut logged = self.logged.lock().unwrap();
            logged.push(Entry {
                time: timestamp,
                event_type: EventType::Input,
                event_data: String::from_utf8(data.to_vec()).unwrap(),
            });
        }

        let mut inner = self.inner.lock().unwrap();
        if inner.is_none() {
            return Err(Box::<dyn Error>::from("You've already exited."));
        }
        let inner = inner.as_mut().unwrap();
        let res = inner.write(data);

        res
    }
}

impl<T> Recorder<T> for Asciicast<T>
where
    T: Tty,
{
    fn begin(&mut self) -> Result<(), Box<dyn Error>> {
        let logged = self.logged.lock();
        if let Err(_) = logged {
            return Err(Box::<dyn Error>::from("Recorder not started."));
        }
        let mut logged = logged.unwrap();
        logged.clear();

        let time = SystemTime::now();
        let begin_time = self.begin_time.lock();
        if let Err(_) = begin_time {
            return Err(Box::<dyn Error>::from("Recorder not started."));
        }
        let mut begin_time = begin_time.unwrap();
        *begin_time = time;

        let begin = self.begin.lock();
        if let Err(_) = begin {
            return Err(Box::<dyn Error>::from("Recorder not started."));
        }
        let mut begin = begin.unwrap();
        *begin = true;
        Ok(())
    }

    fn end(&mut self) -> Result<String, Box<dyn Error>> {
        let begin = self.begin.lock();
        if let Err(_) = begin {
            return Err(Box::<dyn Error>::from("Recorder not started."));
        }
        let mut begin = begin.unwrap();
        if !*begin {
            return Err(Box::<dyn Error>::from("Recorder not started."));
        }
        *begin = false;
        let mut res = String::new();
        let logged = self.logged.lock();
        if let Err(_) = logged {
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
        Ok(res)
    }

    fn swap(&mut self, target: T) -> Result<T, Box<dyn Error>> {
        let mut inner = self.inner.lock().unwrap();
        if inner.is_none() {
            return Err(Box::<dyn Error>::from("You've already exited."));
        }
        let res = inner.take().unwrap();
        *inner = Some(target);
        Ok(res)
    }

    fn exit(self) -> T {
        let mut inner = self.inner.lock().unwrap();
        let inner = inner.take().unwrap();
        inner
    }
}
