pub mod logger;
pub mod util;
pub mod consts;
pub mod term {
    pub mod tty;

    pub mod ssh;
    pub mod serial;
    pub mod shell;

    pub mod recorder;
    pub mod asciicast;
}
pub mod exec {
    pub mod cli_api;
    pub mod cli_exec;
}