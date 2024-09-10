use std::error::Error;

use crate::util::anybase::AnyBase;

pub trait Tty: AnyBase {
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>>;
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn Error>>;
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>>;
}

pub type DynTty = Box<dyn Tty + Send>;

pub trait WrapperTty: Tty {
    fn exit(self) -> DynTty;
}

pub trait InnerTty: WrapperTty {
    fn inner_ref(&self) -> &DynTty;
    fn inner_mut(&mut self) -> &mut DynTty;
}