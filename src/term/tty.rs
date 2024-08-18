use std::error::Error;

pub trait Tty {
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>>;
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn Error>>;
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>>;
}

pub trait WrapperTty<T>: Tty
where
    T: Tty,
{
    fn exit(self) -> T;
    // fn inner(&self) -> Arc<Mutex<T>>; // Due to muti-thread, the ref of the inner is not very easy to get...
    // fn mut_inner(&mut self) -> Arc<Mutex<T>>;
}
