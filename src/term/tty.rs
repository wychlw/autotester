use std::error::Error;

pub trait Tty {
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>>;
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn Error>>;
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>>;
}
