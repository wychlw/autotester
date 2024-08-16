use std::{
    error::Error,
    io::{BufReader, ErrorKind, Read, Write},
    process::{ChildStdin, Command, Stdio},
    sync::{Arc, Mutex},
    thread::{sleep, spawn, JoinHandle},
    time::Duration,
};

use crate::{consts::SHELL_DURATION, logger::{err, log}};

use super::tty::Tty;

pub struct Shell {
    stdin: ChildStdin,
    buff: Arc<Mutex<Vec<u8>>>,
    handle: Option<JoinHandle<()>>,
    stop: Arc<Mutex<bool>>,
}

impl Shell {
    pub fn build(shell: Option<&str>) -> Result<Shell, Box<dyn Error>> {
        let shell = shell.unwrap_or("/bin/sh");

        log(format!("Spawn shell process: {}", shell));

        let inner = Command::new("stdbuf")
            .args(&["-oL", "-eL", shell])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();
        if let Err(e) = inner {
            err(format!("Failed to spawn shell process. Reason: {}", e));
            return Err(Box::new(e));
        }
        let mut inner = inner.unwrap();

        let stdin = inner.stdin.take();
        if let None = stdin {
            err("Failed to get stdin of shell process.");
            return Err(Box::<dyn Error>::from(""));
        }
        let stdin = stdin.unwrap();

        let stdout = inner.stdout.take();
        if let None = stdout {
            err("Failed to get stdout of shell process.");
            return Err(Box::<dyn Error>::from(""));
        }
        let stdout = stdout.unwrap();

        let stderr = inner.stderr.take();
        if let None = stderr {
            err("Failed to get stderr of shell process.");
            return Err(Box::<dyn Error>::from(""));
        }
        let stderr = stderr.unwrap();
        let stdout = stdout.chain(stderr);

        let mut reader = BufReader::new(stdout);

        let mut res = Shell {
            stdin,
            buff: Arc::new(Mutex::new(Vec::new())),
            handle: None,
            stop: Arc::new(Mutex::new(false)),
        };

        let buff_clone = res.buff.clone();
        let stop_clone = res.stop.clone();
        let handle = spawn(move || loop {
            sleep(Duration::from_millis(SHELL_DURATION));
            {
                let stop = stop_clone.lock().unwrap();
                if *stop {
                    log("Stop shell process");
                    return;
                }
            }
            let mut buf = [0u8];
            let sz = reader.read(&mut buf);
            if let Err(e) = sz {
                err(format!("Read from shell process failed. Reason: {}", e));
                break;
            }
            if buf[0] == 0x0 {
                continue;
            }
            let mut buff = buff_clone.lock().unwrap();
            buff.extend_from_slice(&buf);
        });

        res.handle = Some(handle);

        Ok(res)
    }

    fn __stop(&mut self) {
        let stop = self.stop.lock();
        if let Err(e) = stop {
            err(format!("Failed to lock stop mutex. Reason: {}", e));
            return;
        }
        let mut stop = stop.unwrap();
        if *stop {
            return;
        }
        *stop = true;
        log("Try to stop shell process");
        // if let Some(handle) = self.handle.take() {
        //     handle.join().unwrap();
        //     self.inner.wait().unwrap();
        // } // workaround for stopping shell process
    }

    pub fn stop(mut self) {
        self.__stop();
    }
}

impl Tty for Shell {
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut res = Vec::new();
        let mut buff = self.buff.lock().unwrap();
        res.extend(buff.iter());
        buff.clear();
        return Ok(res);
    }
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut res = Vec::new();
        let mut buff = self.buff.lock().unwrap();
        let mut i = 0;
        while i < buff.len() {
            if buff[i] == 0x0A {
                break;
            }
            i += 1;
        }
        if i == buff.len() {
            return Ok(res);
        }
        res.extend_from_slice(&buff[0..i + 1]);
        buff.drain(0..i + 1);
        return Ok(res);
    }
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        loop {
            sleep(Duration::from_millis(SHELL_DURATION));
            match self.stdin.write_all(data) {
                Ok(_) => break,
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => {
                    err(format!("Write to shell process failed. Reason: {}", e));
                    return Err(Box::new(e));
                }
            }
        }
        let res = self.stdin.flush();
        if let Err(e) = res {
            err(format!("Flush to shell process failed. Reason: {}", e));
            return Err(Box::<dyn Error>::from(e));
        }
        return Ok(());
    }
}

impl Drop for Shell {
    fn drop(&mut self) {
        self.__stop();
    }
}
