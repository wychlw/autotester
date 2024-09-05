#![feature(box_into_inner)]

pub mod consts;
pub mod logger;
pub mod term {
    pub mod tty;

    pub mod serial;
    pub mod shell;
    pub mod ssh;

    pub mod asciicast;
    pub mod recorder;
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
    pub mod anybase;
    pub mod util;
}
pub mod pythonapi {
    pub mod shell_like;

    pub mod pyshell;

    // pub mod testapi;
}

use pyo3::prelude::*;
use pythonapi::{pyshell::PyShell, shell_like::PyTty};

#[pymodule]
fn tester(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyTty>()?;
    m.add_class::<PyShell>()?;
    Ok(())
}
