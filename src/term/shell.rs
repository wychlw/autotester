use std::{
    any::Any,
    collections::HashMap,
    env,
    error::Error,
    io::{ErrorKind, Read, Write},
    process::{ChildStdin, Command, Stdio},
    sync::{Arc, Mutex},
    thread::{sleep, spawn, JoinHandle},
    time::Duration,
};

use crate::{consts::SHELL_DURATION, err, info, log, util::anybase::AnyBase};

use super::tty::Tty;

pub struct Shell {
    stdin: ChildStdin,
    buff: Arc<Mutex<Vec<u8>>>,
    handle: Option<JoinHandle<()>>,
    stop: Arc<Mutex<bool>>,
}

impl Shell {
    /**
     * This implementation method is DEFINITELY needs to be changed in the future,
     * at least need to use a stty to let shell HAPPY.
     * But for now... Well, it works.
     * I've already spent too much time trying to make this thing work... Just move on.
     * For now.
     */
    pub fn build(shell: Option<&str>) -> Result<Shell, Box<dyn Error>> {
        let shell = shell.unwrap_or("/bin/sh");

        info!("Spawn shell process: {}", shell);

        let filtered_env: HashMap<String, String> = env::vars()
            .filter(|&(ref k, _)| k == "TERM" || k == "TZ" || k == "LANG" || k == "PATH")
            .collect();

        let inner = Command::new(shell)
            .envs(&filtered_env)
            .envs(Into::<HashMap<_, _>>::into([("PS1", r"[\u@\h \W]\$")]))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();
        if let Err(e) = inner {
            err!("Failed to spawn shell process. Reason: {}", e);
            return Err(Box::new(e));
        }
        let mut inner = inner.unwrap();

        let stdin = inner.stdin.take();
        if let None = stdin {
            err!("Failed to get stdin of shell process.");
            return Err(Box::<dyn Error>::from(""));
        }
        let mut stdin = stdin.unwrap();
        stdin
            .write_all(b"export PS1=\"[\\u@\\h \\W]\\$\"\n")
            .unwrap();

        let stdout = inner.stdout.take();
        if let None = stdout {
            err!("Failed to get stdout of shell process.");
            return Err(Box::<dyn Error>::from(""));
        }
        let stdout = stdout.unwrap();

        let stderr = inner.stderr.take();
        if let None = stderr {
            err!("Failed to get stderr of shell process.");
            return Err(Box::<dyn Error>::from(""));
        }
        let stderr = stderr.unwrap();
        let mut stdout = stdout.chain(stderr);

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
                    log!("Stop shell process");
                    return;
                }
            }
            let mut buf = [0u8];
            let sz = stdout.read(&mut buf);
            if let Err(e) = sz {
                err!("Read from shell process failed. Reason: {}", e);
                break;
            }
            if buf[0] == 0x0 {
                continue;
            }
            let mut buff = buff_clone.lock().unwrap();
            if buf[0] != 0x0 {
                buff.extend_from_slice(&buf);
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
        return Ok(res);
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
        return Ok(res);
    }
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        loop {
            sleep(Duration::from_millis(SHELL_DURATION));
            match self.stdin.write_all(data) {
                Ok(_) => break,
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => {
                    err!("Write to shell process failed. Reason: {}", e);
                    return Err(Box::new(e));
                }
            }
        }
        let res = self.stdin.flush();
        if let Err(e) = res {
            err!("Flush to shell process failed. Reason: {}", e);
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
