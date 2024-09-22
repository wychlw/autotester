use std::{
    error::Error,
    process::Output,
    sync::{Arc, Mutex},
};

use crate::info;

use super::devhost::DevHost;

#[derive(Default)]
pub struct Sdwirec {}

#[allow(static_mut_refs)]
impl DevHost<Sdwirec> for Sdwirec {
    fn get_device() -> Arc<Mutex<Sdwirec>> {
        static mut DEVICE: Option<Arc<Mutex<Sdwirec>>> = None;
        unsafe {
            DEVICE
                .get_or_insert_with(|| Arc::new(Mutex::new(Sdwirec::default())))
                .clone()
        }
    }
}

pub enum SdwirecStat {
    TS,
    DUT,
}

pub enum SdwirecChooser {
    Id(u16),
    Serial(String),
    Vendor(u16),
    Product(u16),
}

impl Sdwirec {
    fn format_device(&self, chooser: &SdwirecChooser) -> String {
        match chooser {
            SdwirecChooser::Id(x) => {
                format!("-v {} ", x)
            }
            SdwirecChooser::Serial(x) => {
                format!("-e {} ", x)
            }
            SdwirecChooser::Vendor(x) => {
                format!("-x {:#04x} ", x)
            }
            SdwirecChooser::Product(x) => {
                format!("-a {:#04x} ", x)
            }
        }
    }

    fn try_run(&self, cmd: &str) -> Result<Output, Box<dyn std::error::Error>> {
        let res = std::process::Command::new("sh").arg("-c").arg(cmd).output();
        if let Err(e) = res {
            return Err(Box::new(e));
        }
        let res = res.unwrap();
        Ok(res)
    }

    pub fn get_stat(
        &self,
        chooser: &SdwirecChooser,
    ) -> Result<SdwirecStat, Box<dyn std::error::Error>> {
        let mut cmd = String::from("sudo sd-mux-ctrl ");
        cmd += &self.format_device(chooser);
        cmd += "-u";

        let res = self.try_run(&cmd)?;
        if !res.status.success() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Failed to get status of device. Reason: {}",
                    String::from_utf8(res.stderr).unwrap()
                ),
            )));
        }

        /*
         * Format:
         *  "SD connected to: [TS|DUT]"
         *  "Unable to open ftdi device: [Reason]"
         */
        let res = String::from_utf8(res.stdout).unwrap();
        if res.contains("SD connected to: TS") {
            Ok(SdwirecStat::TS)
        } else if res.contains("SD connected to: DUT") {
            Ok(SdwirecStat::DUT)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to get status of device. Reason: {}", res),
            )))
        }
    }

    pub fn to_ts(&self, chooser: &SdwirecChooser) -> Result<(), Box<dyn Error>> {
        if let SdwirecStat::TS = self.get_stat(chooser)? {
            return Ok(());
        }

        let mut cmd = String::from("sudo sd-mux-ctrl ");
        cmd += &self.format_device(chooser);
        cmd += "-ts";

        let res = self.try_run(&cmd)?;
        if !res.status.success() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Failed to switch device to TS. Reason: {}",
                    String::from_utf8(res.stderr).unwrap()
                ),
            )));
        }
        info!("Switched device {} to TS", self.format_device(chooser));
        Ok(())
    }

    pub fn to_dut(&self, chooser: &SdwirecChooser) -> Result<(), Box<dyn Error>> {
        if let SdwirecStat::DUT = self.get_stat(chooser)? {
            return Ok(());
        }

        let mut cmd = String::from("sudo sd-mux-ctrl ");
        cmd += &self.format_device(chooser);
        cmd += "-d";

        let res = self.try_run(&cmd)?;
        if !res.status.success() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Failed to switch device to DUT. Reason: {}",
                    String::from_utf8(res.stderr).unwrap()
                ),
            )));
        }
        info!("Switched device {} to DUT", self.format_device(chooser));
        Ok(())
    }

    pub fn set_to(
        &self,
        chooser: &SdwirecChooser,
        stat: &SdwirecStat,
    ) -> Result<(), Box<dyn Error>> {
        match stat {
            SdwirecStat::TS => self.to_ts(chooser),
            SdwirecStat::DUT => self.to_dut(chooser),
        }
    }
}

pub struct SdwirecProd {
    chooser: SdwirecChooser,
}

impl SdwirecProd {
    pub fn new(chooser: SdwirecChooser) -> SdwirecProd {
        SdwirecProd { chooser }
    }

    pub fn get_stat(&self) -> Result<SdwirecStat, Box<dyn Error>> {
        let sdwirec = Sdwirec::get_device();
        let sdwirec = sdwirec.lock().unwrap();
        sdwirec.get_stat(&self.chooser)
    }

    pub fn to_ts(&self) -> Result<(), Box<dyn Error>> {
        let sdwirec = Sdwirec::get_device();
        let sdwirec = sdwirec.lock().unwrap();
        sdwirec.to_ts(&self.chooser)
    }

    pub fn to_dut(&self) -> Result<(), Box<dyn Error>> {
        let sdwirec = Sdwirec::get_device();
        let sdwirec = sdwirec.lock().unwrap();
        sdwirec.to_dut(&self.chooser)
    }

    pub fn set_to(&self, stat: &SdwirecStat) -> Result<(), Box<dyn Error>> {
        let sdwirec = Sdwirec::get_device();
        let sdwirec = sdwirec.lock().unwrap();
        sdwirec.set_to(&self.chooser, stat)
    }
}
