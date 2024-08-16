use std::error::Error;
use std::io::ErrorKind;
use std::thread::sleep;
use std::time::Duration;

use serialport::{self, SerialPort};

use crate::consts::SHELL_DURATION;
use crate::logger::err;
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
        let mut buf = Vec::new();
        loop {
            sleep(Duration::from_millis(SHELL_DURATION));
            let mut buff = [0u8];
            match self.inner.read(&mut buff) {
                Ok(_) => {
                    buf.extend_from_slice(&buff);
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
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buf = Vec::new();
        loop {
            sleep(Duration::from_millis(SHELL_DURATION));
            let mut buff = [0u8];
            match self.inner.read(&mut buff) {
                Ok(_) => {
                    if buff[0] == 0x0A {
                        buf.extend_from_slice(&buff);
                        return Ok(buf);
                    }
                    buf.extend_from_slice(&buff);
                }
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => {
                    err(format!("Read line from serial port failed. Reason: {}", e));
                    return Err(Box::new(e));
                }
            }
        }
    }
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        loop {
            sleep(Duration::from_millis(SHELL_DURATION));
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
