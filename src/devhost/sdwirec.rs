use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use super::devhost::DevHost;

pub struct Sdwirec {}

impl Sdwirec {
    pub fn new() -> Sdwirec {
        Sdwirec {}
    }
}

impl DevHost<Sdwirec> for Sdwirec {
    fn get_device() -> Arc<Mutex<Sdwirec>> {
        static mut DEVICE: Option<Arc<Mutex<Sdwirec>>> = None;
        unsafe {
            DEVICE
                .get_or_insert_with(|| Arc::new(Mutex::new(Sdwirec::new())))
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
    pub fn get_stat(
        &self,
        chooser: &SdwirecChooser,
    ) -> Result<SdwirecStat, Box<dyn std::error::Error>> {
        match chooser {
            SdwirecChooser::Id(x) => {
                todo!();
            }
            SdwirecChooser::Serial(x) => {
                todo!();
            }
            SdwirecChooser::Vendor(x) => {
                todo!();
            }
            SdwirecChooser::Product(x) => {
                todo!();
            }
        }
    }

    pub fn to_ts(&self, chooser: &SdwirecChooser) -> Result<(), Box<dyn Error>> {
        match chooser {
            SdwirecChooser::Id(x) => {
                todo!();
            }
            SdwirecChooser::Serial(x) => {
                todo!();
            }
            SdwirecChooser::Vendor(x) => {
                todo!();
            }
            SdwirecChooser::Product(x) => {
                todo!();
            }
        }
    }

    pub fn to_dut(&self, chooser: &SdwirecChooser) -> Result<(), Box<dyn Error>> {
        match chooser {
            SdwirecChooser::Id(x) => {
                todo!();
            }
            SdwirecChooser::Serial(x) => {
                todo!();
            }
            SdwirecChooser::Vendor(x) => {
                todo!();
            }
            SdwirecChooser::Product(x) => {
                todo!();
            }
        }
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
