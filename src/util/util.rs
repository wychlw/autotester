use std::{
    error::Error,
    io::{BufRead, ErrorKind},
};

use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::info;

#[macro_export]
macro_rules! todo {
    () => {
        Err(Box::<dyn Error>::from("TODO"))
    };
}

#[macro_export]
macro_rules! unfinished {
    () => {
        Err(Box::<dyn Error>::from("UNFINISHED"))
    };
}

pub fn rand_string(len: usize) -> Vec<u8> {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .collect::<Vec<u8>>()
}

pub fn try_read<R: BufRead + ?Sized>(
    r: &mut R,
    buf: &mut Vec<u8>,
) -> Result<usize, Box<dyn Error>> {
    loop {
        let used = {
            let available = match r.fill_buf() {
                Ok(n) => n,
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Err(Box::new(e)),
            };
            buf.extend_from_slice(available);
            available.len()
        };
        r.consume(used);
        return Ok(used);
    }
}
