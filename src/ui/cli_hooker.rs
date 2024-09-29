use std::{
    error::Error,
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
    thread::{sleep, spawn, JoinHandle},
    time::Duration,
};

use pyo3::exceptions::PyRuntimeError;

use crate::{
    cli::tty::Tty,
    consts::DURATION,
    err, impl_any,
    pythonapi::shell_like::{PyTtyWrapper, TtyType},
};

use super::terminal::TerminalMessage;

pub struct UiCliWrapper {
    inner: Arc<Mutex<Option<Box<TtyType>>>>,
    buf: Arc<Mutex<Vec<u8>>>,
    sender: Arc<Mutex<Option<Sender<TerminalMessage>>>>,
    receiver: Arc<Mutex<Option<Receiver<TerminalMessage>>>>,
    stop: Arc<Mutex<bool>>,
    handle: Option<JoinHandle<()>>,
}

impl UiCliWrapper {
    pub fn build(inner: *mut TtyType) -> Self {
        let buf = Arc::new(Mutex::new(Vec::new()));
        let sender = Arc::new(Mutex::new(None));
        let receiver = Arc::new(Mutex::new(None));
        let inner = Some(unsafe { Box::from_raw(inner) });
        let mut res = Self {
            inner: Arc::new(Mutex::new(inner)),
            buf,
            sender,
            receiver,
            stop: Arc::new(Mutex::new(false)),
            handle: None,
        };

        let inner = res.inner.clone();
        let buf = res.buf.clone();
        let sender = res.sender.clone();
        let receiver = res.receiver.clone();
        let stop = res.stop.clone();

        let handler = spawn(move || loop {
            sleep(Duration::from_millis(DURATION));
            {
                let stop = stop.lock().unwrap();
                if *stop {
                    break;
                }
            }
            // read into buffer
            {
                let mut inner = inner.lock().unwrap();
                if inner.is_none() {
                    continue;
                }
                let inner = inner.as_mut().unwrap();
                let data = inner.read();
                if let Err(e) = data {
                    err!("read error: {}", e);
                    break;
                }
                let data = data.unwrap();
                let mut buf = buf.lock().unwrap();
                buf.extend(data);
            }
            // recv stop signal from terminal
            {
                let mut recv_o = receiver.lock().unwrap();
                if recv_o.is_none() {
                    continue;
                }
                let recv = recv_o.as_mut().unwrap();
                match recv.try_recv() {
                    Ok(TerminalMessage::Data(_)) => {
                        unreachable!()
                    }
                    Ok(TerminalMessage::Close) => {
                        let mut send_o = sender.lock().unwrap();
                        let send = send_o.as_ref().unwrap();
                        send.send(TerminalMessage::Close).unwrap();
                        send_o.take();
                        recv_o.take();
                    }
                    Err(_) => {}
                }
            }
        });

        res.handle = Some(handler);

        res
    }
}

impl Drop for UiCliWrapper {
    fn drop(&mut self) {
        let mut stop = self.stop.lock().unwrap();
        *stop = true;
    }
}
impl_any!(UiCliWrapper);
impl Tty for UiCliWrapper {
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buf = self.buf.lock().unwrap();
        let res = buf.clone();
        buf.clear();
        Ok(res)
    }
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buf = self.buf.lock().unwrap();
        let mut res = Vec::new();
        for &c in buf.iter() {
            if c == b'\n' {
                break;
            }
            res.push(c);
        }
        buf.drain(..res.len());
        Ok(res)
    }
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let mut inner = self.inner.lock().unwrap();
        if inner.is_none() {
            return Err("tty is closed".into());
        }
        let inner = inner.as_mut().unwrap();
        inner.write(data)
    }
}

pub struct PyUiCliWrapper {
    inner: TtyType,
}

impl PyUiCliWrapper {
    fn inner_spec(&self) -> &Box<UiCliWrapper> {
        let inner = &self.inner;
        inner.as_any().downcast_ref::<Box<UiCliWrapper>>().unwrap()
    }
    fn inner_spec_mut(&mut self) -> &mut Box<UiCliWrapper> {
        let inner = &mut self.inner;
        inner
            .as_any_mut()
            .downcast_mut::<Box<UiCliWrapper>>()
            .unwrap()
    }
}

impl PyTtyWrapper for PyUiCliWrapper {
    fn take(&mut self) -> pyo3::PyResult<*mut TtyType> {
        let inner = self.inner_spec_mut();
        let mut inner = inner.inner.lock().unwrap();
        let inner = inner.take();
        if inner.is_none() {
            return Err(PyRuntimeError::new_err("tty is closed"));
        }
        let inner = inner.unwrap();
        let inner = Box::into_raw(inner);
        Ok(inner)
    }
    fn safe_take(&mut self) -> pyo3::PyResult<Box<TtyType>> {
        let res = self.take()?;
        Ok(unsafe { Box::from_raw(res) })
    }
    fn get(&self) -> pyo3::PyResult<&TtyType> {
        let inner = self.inner_spec();
        let inner = inner.inner.lock().unwrap();
        if inner.is_none() {
            return Err(PyRuntimeError::new_err("tty is closed"));
        }
        Ok(&self.inner)
    }
    fn get_mut(&mut self) -> pyo3::PyResult<&mut TtyType> {
        {
            let inner = self.inner_spec();
            let inner = inner.inner.lock().unwrap();
            if inner.is_none() {
                return Err(PyRuntimeError::new_err("tty is closed"));
            }
        }
        Ok(&mut self.inner)
    }
    fn put(&mut self, tty: *mut TtyType) -> pyo3::PyResult<()> {
        let inner = self.inner_spec_mut();
        let mut inner = inner.inner.lock().unwrap();
        if inner.is_some() {
            return Err(PyRuntimeError::new_err("tty is already opened"));
        }
        let tty = unsafe { Box::from_raw(tty) };
        *inner = Some(tty);
        Ok(())
    }
}
