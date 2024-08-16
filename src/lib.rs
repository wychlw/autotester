pub mod logger;
pub mod util;
pub mod consts;
pub mod term {
    pub mod asciicast;
    pub mod recorder;
    pub mod serial;
    pub mod shell;
    pub mod tty;
    // pub mod pipereader;
}
pub mod exec {
    pub mod cli_api;
    pub mod cli_exec;
}