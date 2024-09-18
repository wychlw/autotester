use std::{
    any::Any,
    error::Error,
    io::{ErrorKind, Read, Write},
    net::TcpStream,
    path::Path,
    sync::{Arc, Mutex},
    thread::{sleep, spawn, JoinHandle},
    time::Duration,
};

use ssh2::Channel;

use crate::{
    consts::SHELL_DURATION, err, log, util::anybase::AnyBase
};

use super::tty::Tty;

#[derive(Clone)]
pub enum SshPass {
    Password(String),
    Key(String), // Path to private key
}

pub struct Ssh {
    // sess: Session,
    channel: Arc<Mutex<Channel>>,
    buff: Arc<Mutex<Vec<u8>>>,
    stop: Arc<Mutex<bool>>,
    handle: Option<JoinHandle<()>>,
}

impl Ssh {
    fn connect(
        host: &str,
        port: u16,
        user: &str,
        pass: SshPass,
    ) -> Result<ssh2::Session, Box<dyn Error>> {
        let tcp = TcpStream::connect(format!("{}:{}", host, port))?;
        let mut sess = ssh2::Session::new()?;
        sess.set_tcp_stream(tcp);
        sess.handshake()?;
        match pass {
            SshPass::Password(ref pass) => {
                sess.userauth_password(user, pass)?;
            }
            SshPass::Key(ref key) => {
                sess.userauth_pubkey_file(user, None, Path::new(&key), None)?;
            }
        }
        Ok(sess)
    }

    pub fn build(host: &str, port: u16, user: &str, pass: SshPass) -> Ssh {
        let sess = Self::connect(host, port, user, pass);
        if let Err(e) = sess {
            panic!("Failed to connect to SSH server. Reason: {}", e);
        }
        let sess = sess.unwrap();

        let channel = sess.channel_session();
        if let Err(e) = channel {
            panic!("Failed to open SSH channel. Reason: {}", e);
        }
        let mut channel = channel.unwrap();

        let e = channel.shell();
        if let Err(e) = e {
            err!("Failed to open SSH shell. Reason: {}", e);
        }

        let channel = Arc::new(Mutex::new(channel));
        let buff = Arc::new(Mutex::new(Vec::new()));
        let stop = Arc::new(Mutex::new(false));

        let channel_clone = channel.clone();
        let buff_clone = buff.clone();
        let stop_clone = stop.clone();

        let handle = spawn(move || loop {
            let stop = stop_clone.lock().unwrap();
            if *stop {
                log!("Stop SSH shell.");
                break;
            }

            let mut channel = channel_clone.lock().unwrap();
            let mut buf = [0u8];
            let sz = channel.read(&mut buf);
            if let Err(e) = sz {
                err!("Read from SSH channel failed. Reason: {}", e);
                break;
            }
            if buf[0] == 0x0 {
                continue;
            }
            let mut buff = buff_clone.lock().unwrap();
            buff.extend_from_slice(&buf);
        });

        Ssh {
            // sess: sess,
            channel: channel,
            buff,
            stop,
            handle: Some(handle),
        }
    }

    pub fn exit(self) {
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
        log!("Try to stop SSH shell.");
        // if let Some(handle) = self.handle.take() {
        //     handle.join().unwrap();
        //     self.inner.wait().unwrap();
        // } // workaround for stopping shell process
    }
}

impl AnyBase for Ssh {
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

impl Tty for Ssh {
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut res = Vec::new();
        let mut buff = self.buff.lock().unwrap();
        res.extend_from_slice(&buff);
        buff.clear();
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
        loop {
            sleep(Duration::from_millis(SHELL_DURATION));
            let mut channel = self.channel.lock().unwrap();
            match channel.write_all(data) {
                Ok(_) => break,
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => {
                    err!("Write to shell process failed. Reason: {}", e);
                    return Err(Box::new(e));
                }
            }
        }
        let mut channel = self.channel.lock().unwrap();
        let res = channel.flush();
        if let Err(e) = res {
            err!("Flush to shell process failed. Reason: {}", e);
            return Err(Box::<dyn Error>::from(e));
        }
        Ok(())
    }
}
