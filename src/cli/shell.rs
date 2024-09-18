use ptyprocess::{stream::Stream, PtyProcess};
use std::io::BufReader;
use std::ops::DerefMut;
use std::os::fd::AsRawFd;
use std::{
    any::Any,
    error::Error,
    io::Write,
    process::Command,
    sync::{Arc, Mutex},
    thread::{sleep, spawn, JoinHandle},
    time::Duration,
};

use crate::util::util::try_read;
use crate::{consts::SHELL_DURATION, err, info, log, util::anybase::AnyBase};

use super::tty::Tty;

pub struct Shell {
    inner: Arc<Mutex<Stream>>,
    buff: Arc<Mutex<Vec<u8>>>,
    proc: PtyProcess,
    handle: Option<JoinHandle<()>>,
    stop: Arc<Mutex<bool>>,
}

impl Shell {
    pub fn build(shell: Option<&str>) -> Result<Shell, Box<dyn Error>> {
        let shell = shell.unwrap_or("/bin/sh");

        info!("Spawn shell process: {}", shell);

        let mut inner = Command::new(shell);
        inner.args(["-i"]);
        let inner = PtyProcess::spawn(inner);
        if let Err(e) = inner {
            err!("Failed to spawn shell process. Reason: {}", e);
            return Err(Box::new(e));
        }
        let proc = inner.unwrap();
        let inner = proc.get_pty_stream()?;

        info!(
            "Shell process spawned, got streamed... FD: {:?}",
            inner.as_raw_fd()
        );

        let inner = Arc::new(Mutex::new(inner));

        let mut res = Shell {
            inner,
            buff: Arc::new(Mutex::new(Vec::new())),
            proc,
            handle: None,
            stop: Arc::new(Mutex::new(false)),
        };

        let buff = res.buff.clone();
        let stop = res.stop.clone();
        let stream = res.inner.clone();
        let handle = spawn(move || loop {
            sleep(Duration::from_millis(SHELL_DURATION));
            {
                let stop: std::sync::MutexGuard<'_, bool> = stop.lock().unwrap();
                if *stop {
                    log!("Stop shell process");
                }
            }
            let mut buf = Vec::new();
            {
                let mut stream = stream.lock().unwrap();
                let mut reader = BufReader::new(stream.deref_mut());
                let sz = try_read(&mut reader, &mut buf);
                if let Err(e) = sz {
                    err!("Failed to read from shell process. Reason: {}", e);
                    return;
                }
                let sz = sz.unwrap();
                if sz == 0 {
                    continue;
                }
            }
            {
                let mut buff = buff.lock().unwrap();
                buff.extend(buf.iter());
            }
        });

        res.handle = Some(handle);

        Ok(res)
    }

    fn __stop(&mut self) {
        let stop = self.stop.lock();
        if let Err(e) = stop {
            err!("Failed to lock stop mutex. Reason: {}", e);
            return;
        }
        let mut stop = stop.unwrap();
        if *stop {
            return;
        }
        *stop = true;
        log!("Try to stop shell process");
        self.proc.exit(false).unwrap();
        // if let Some(handle) = self.handle.take() {
        //     handle.join().unwrap();
        //     self.inner.wait().unwrap();
        // } // workaround for stopping shell process
    }

    pub fn stop(mut self) {
        self.__stop();
    }
}

impl AnyBase for Shell {
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

impl Tty for Shell {
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut res = Vec::new();
        let mut buff = self.buff.lock().unwrap();
        res.extend(buff.iter());
        buff.clear();
        if !res.is_empty() {
            log!("Shell read: {:?}", String::from_utf8_lossy(&res));
        }
        Ok(res)
    }
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut res = Vec::new();
        loop {
            sleep(Duration::from_millis(SHELL_DURATION));
            let mut buff = self.buff.lock().unwrap();
            if buff.is_empty() {
                continue;
            }
            let mut i = 0;
            while i < buff.len() {
                res.push(buff[i]);
                i += 1;
                if res.ends_with(&[0x0A]) {
                    break;
                }
            }
            buff.drain(0..i);
            if res.ends_with(&[0x0A]) {
                break;
            }
        }
        Ok(res)
    }
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let mut stream = self.inner.lock().unwrap();
        info!("Shell locked...");
        match stream.write_all(data) {
            Ok(_) => {
                stream.flush().unwrap();
                info!("Shell write: {:?}", String::from_utf8_lossy(data));
                Ok(())
            }
            Err(e) => {
                err!("Write to shell process failed. Reason: {}", e);
                Err(Box::new(e))
            }
        }
    }
}

impl Drop for Shell {
    fn drop(&mut self) {
        self.__stop();
    }
}
