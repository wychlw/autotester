use std::any::Any;
use std::error::Error;
use std::io::ErrorKind;
use std::thread::sleep;
use std::time::Duration;

use serialport::{self, SerialPort};

use crate::consts::SHELL_DURATION;
use crate::cli::tty::Tty;
use crate::util::anybase::AnyBase;
use crate::{err, info};

pub struct Serial {
    inner: Box<dyn SerialPort>,
}

impl Serial {
    pub fn build(port: &str, baud: u32) -> Result<Serial, Box<dyn Error>> {
        let inner = serialport::new(port, baud)
            .timeout(Duration::from_millis(50))
            .open();

        if let Err(e) = inner {
            err!("Open serial port failed! Reason: {}", e);
            return Err(Box::new(e));
        }

        info!("Serial port opened: {} at baud rate {}", port, baud);

        Ok(Serial {
            inner: inner.unwrap(),
        })
    }
}

impl AnyBase for Serial {
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

impl Tty for Serial {
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buf = Vec::new();
        sleep(Duration::from_millis(SHELL_DURATION));
        loop {
            let mut buff = [0u8];
            match self.inner.read(&mut buff) {
                Ok(_) => {
                    if buff[0] == 0x0 {
                        return Ok(buf);
                    }
                    buf.extend_from_slice(&buff);
                }
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) if e.kind() == ErrorKind::TimedOut => return Ok(buf),
                Err(e) => {
                    err!("Read from serial port failed. Reason: {}", e);
                    return Err(Box::new(e));
                }
            }
        }
    }
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buf = Vec::new();
        sleep(Duration::from_millis(SHELL_DURATION));
        loop {
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
                    err!("Read line from serial port failed. Reason: {}", e);
                    return Err(Box::new(e));
                }
            }
        }
    }
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        loop {
            sleep(Duration::from_millis(SHELL_DURATION));
            self.inner.flush()?;
            match self.inner.write_all(data) {
                Ok(_) => {
                    self.inner.flush()?;
                    break;
                },
                Err(e) if e.kind() == ErrorKind::Interrupted => {
                    info!("Write being Interrupted!");
                    continue;
                },
                Err(e) => {
                    err!("Write to serial port failed. Reason: {}", e);
                    return Err(Box::new(e));
                }
            }
        }

        Ok(())
    }
}
