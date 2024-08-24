
use rand::{distributions::Alphanumeric, thread_rng, Rng};

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
    let rnd = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .collect::<Vec<u8>>();

    rnd
}
