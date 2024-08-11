
#[allow(unused)]
pub fn log<T: std::fmt::Display>(msg: T) {
    println!("{}", msg);
}

#[allow(unused)]
pub fn warn<T: std::fmt::Display>(msg: T) {
    eprintln!("{}", msg);
}

#[allow(unused)]
pub fn err<T: std::fmt::Display>(msg: T) {
    eprintln!("{}", msg);
}