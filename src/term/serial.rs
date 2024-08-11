use std::error::Error;
use std::io::ErrorKind;

use serialport::{self, SerialPort};

use crate::logger::{err, log};
use crate::term::tty::Tty;

pub struct Serial {
    inner: Box<dyn SerialPort>,
}

impl Serial {
    pub fn build(port: &str, baud: u32) -> Result<Serial, Box<dyn Error>> {
        let inner = serialport::new(port, baud).open();

        if let Err(e) = inner {
            err(format!("Open serial port failed! Reason: {}", e));
            return Err(Box::new(e));
        }

        return Ok(Serial {
            inner: inner.unwrap(),
        });
    }
}

impl Tty for Serial {
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buf = Vec::with_capacity(1024);
        loop {
            match self.inner.read(&mut buf) {
                Ok(sz) => {
                    log(format!("Read from serial port, len {}: {:?}", sz, buf));
                    return Ok(buf);
                }
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => {
                    err(format!("Read from serial port failed. Reason: {}", e));
                    return Err(Box::new(e));
                }
            }
        }
    }

    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        loop {
            match self.inner.write_all(data) {
                Ok(_) => break,
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => {
                    err(format!("Write to serial port failed. Reason: {}", e));
                    return Err(Box::new(e));
                }
            }
        }

        return Ok(());
    }
}
