use std::{
    error::Error,
    io::{BufRead, BufReader, Chain, ErrorKind, Read, Write},
    process::{Child, ChildStderr, ChildStdin, ChildStdout, Command, Stdio},
    sync::{Arc, Mutex},
    thread::{spawn, JoinHandle},
};

use crate::logger::{err, log};

use super::tty::Tty;

pub struct Shell {
    inner: Child,
    stdin: ChildStdin,
    buff: Arc<Mutex<Vec<u8>>>,
    handle: Option<JoinHandle<()>>,
}

impl Shell {
    pub fn build(shell: Option<&str>) -> Result<Shell, Box<dyn Error>> {
        let shell = shell.unwrap_or("/bin/sh");

        let inner = Command::new("stdbuf")
            .args(&["-oL", "-eL", shell, "-i"])
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
            inner,
            stdin,
            buff: Arc::new(Mutex::new(Vec::new())),
            handle: None,
        };

        let buff_clone = res.buff.clone();
        let handle = spawn(move || {
            let mut buff = buff_clone.lock().unwrap();
            loop {
                let mut buf = [0; 10];
                let sz = reader.read(&mut buf);
                if let Err(e) = sz {
                    err(format!("Read from shell process failed. Reason: {}", e));
                    break;
                }
                let sz = sz.unwrap();
                // log(format!("Read from shell process, len {}: {:?}", sz, buf));
                buff.extend_from_slice(&buf);
            }
        });

        res.handle = Some(handle);

        Ok(res)
    }
}

impl Tty for Shell {
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut res = Vec::new();
        let mut buff = self.buff.lock().unwrap();
        res.extend(buff.iter());
        buff.clear();
        log(format!(
            "Read from shell process, len {}: {:?}",
            res.len(),
            String::from_utf8(res.clone()).unwrap()
        ));
        return Ok(res);
    }
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        loop {
            match self.stdin.write_all(data) {
                Ok(_) => {
                    log(format!(
                        "Write to shell process, len {}: {:?}",
                        data.len(),
                        String::from_utf8(data.to_vec()).unwrap()
                    ));
                    break;
                }
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
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
            self.inner.wait().unwrap();
        }
    }
}
