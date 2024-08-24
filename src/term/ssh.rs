use std::{any::Any, error::Error, io::Write, net::TcpStream, path::Path};

use ssh2::{Channel, Session};

use crate::{logger::err, util::anybase::AnyBase};

use super::tty::Tty;

#[derive(Clone)]
pub enum SshPass {
    Password(String),
    Key(String), // Path to private key
}

pub struct Ssh {
    sess: Option<Session>,
    channel: Option<Channel>,
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
            err(format!("Failed to open SSH shell. Reason: {}", e));
        }
        Ssh {
            sess: Some(sess),
            channel: Some(channel),
        }
    }

    pub fn exit(self) {
        if let Some(sess) = self.sess {
            let e = sess.disconnect(None, "Bye", None);
            if let Err(e) = e {
                err(format!("Failed to disconnect SSH session. Reason: {}", e));
            }
        }
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
        todo!();
    }
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        if self.channel.is_none() {
            return Err(Box::from("SSH channel is not opened"));
        }
        let channel = self.channel.as_mut().unwrap();
        todo!();
    }
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        if self.channel.is_none() {
            return Err(Box::from("SSH channel is not opened"));
        }
        let channel = self.channel.as_mut().unwrap();
        let e = channel.write(data);
        if let Err(e) = e {
            err(format!("Failed to write to SSH channel. Reason: {}", e));
            return Err(Box::from(e));
        }
        Ok(())
    }
}
