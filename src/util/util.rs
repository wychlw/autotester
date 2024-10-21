use std::{
    error::Error,
    io::{BufRead, ErrorKind},
};

use rand::{distributions::Alphanumeric, thread_rng, Rng};

#[macro_export]
macro_rules! unfinished {
    () => {
        Err("UNFINISHED".into())
    };
}

pub fn rand_u8(len: usize) -> Vec<u8> {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .collect::<Vec<u8>>()
}


pub fn rand_string(len: usize) -> String {
    String::from_utf8(rand_u8(len)).unwrap()
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
