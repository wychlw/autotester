#![feature(box_into_inner)]

pub mod logger;
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
    pub mod cli_exec_sudo;

    pub mod runner;
}
pub mod devhost {
    pub mod devhost;
    pub mod sdwirec;
}
pub mod device {
    pub mod device;
}

pub mod util {
    pub mod util;
    pub mod anybase;
}

pub mod pythonapi {
    pub mod shell_like;
}