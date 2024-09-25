use portable_pty::{native_pty_system, Child, CommandBuilder, PtyPair, PtySize};
use std::io::{BufReader, Read};
use std::ops::DerefMut;
use std::{
    error::Error,
    io::Write,
    sync::{Arc, Mutex},
    thread::{sleep, spawn, JoinHandle},
    time::Duration,
};

use crate::impl_any;
use crate::util::util::try_read;
use crate::{consts::SHELL_DURATION, err, info, log};

use super::tty::Tty;

#[allow(dead_code)]
pub struct Shell {
    buff: Arc<Mutex<Vec<u8>>>,
    pty: PtyPair,                        // unused: As holder
    child: Box<dyn Child + Send + Sync>, // unused: As holder
    reader: Arc<Mutex<Box<dyn Read + Send>>>,
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
    handle: Option<JoinHandle<()>>,
    stop: Arc<Mutex<bool>>,
}

impl Shell {
    pub fn build(shell: Option<&str>) -> Result<Shell, Box<dyn Error>> {
        let shell = shell.unwrap_or("/bin/sh");

        info!("Spawn shell process: {}", shell);

        let mut cmd = CommandBuilder::new(shell);
        cmd.arg("-i");
        let pty_system = native_pty_system();
        let pty = pty_system.openpty(PtySize::default())?;

        let child = pty.slave.spawn_command(cmd)?;

        let reader = pty.master.try_clone_reader()?;

        let writer = pty.master.take_writer()?;

        let mut res = Shell {
            buff: Arc::new(Mutex::new(Vec::new())),
            pty,
            child,
            reader: Arc::new(Mutex::new(reader)),
            writer: Arc::new(Mutex::new(writer)),
            handle: None,
            stop: Arc::new(Mutex::new(false)),
        };

        let buff = res.buff.clone();
        let stop = res.stop.clone();
        let reader = res.reader.clone();
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
                let mut reader = reader.lock().unwrap();
                let mut r = BufReader::new(reader.deref_mut());
                let sz = try_read(&mut r, &mut buf);
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
        // writeln!(self.pty.master.take_writer().unwrap(), "exit").unwrap();
        // self.child.kill().unwrap();
        // if let Some(handle) = self.handle.take() {
        //     handle.join().unwrap();
        //     self.inner.wait().unwrap();
        // } // workaround for stopping shell process
    }

    pub fn stop(mut self) {
        self.__stop();
    }
}

impl_any!(Shell);

impl Tty for Shell {
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut res = Vec::new();
        let buff = self.buff.clone();
        let mut buff = buff.lock().unwrap();
        res.extend(buff.iter());
        buff.clear();
        Ok(res)
    }
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut res = Vec::new();
        let buff = self.buff.clone();
        loop {
            sleep(Duration::from_millis(SHELL_DURATION));
            {
                let mut buff = buff.lock().unwrap();
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
        }
        Ok(res)
    }
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let writer = self.writer.clone();
        let mut writer = writer.lock().unwrap();
        match writer.write_all(data) {
            Ok(_) => {
                writer.flush().unwrap();
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
