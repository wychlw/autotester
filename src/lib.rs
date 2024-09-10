#![feature(box_into_inner)]

pub mod consts;
pub mod term {
    pub mod tty;

    pub mod serial;
    pub mod shell;
    pub mod ssh;

    pub mod asciicast;
    pub mod recorder;
    pub mod tee;
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
    pub mod logger;
    pub mod singleton;
    pub mod util;
}
pub mod pythonapi {
    pub mod shell_like;

    pub mod pyexec;
    pub mod pyshell;
    pub mod pytee;

    pub mod pyhook;

    pub mod util;
}

use pyo3::prelude::*;
use pythonapi::{
    pyexec::PyExec,
    pyhook::build_ttyhook,
    pyshell::PyShell,
    pytee::PyTee,
    shell_like::PyTty,
    util::{get_log_level, set_log_level},
};

#[pymodule]
#[pyo3(name = "tester")]
fn tester(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyTty>()?;
    m.add_class::<PyShell>()?;
    m.add_class::<PyTee>()?;
    m.add_class::<PyExec>()?;
    m.add_function(wrap_pyfunction!(build_ttyhook, m)?)?;
    m.add_function(wrap_pyfunction!(set_log_level, m)?)?;
    m.add_function(wrap_pyfunction!(get_log_level, m)?)?;
    Ok(())
}
