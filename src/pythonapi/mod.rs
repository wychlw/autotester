pub mod shell_like;

pub mod pyexec;
pub mod pyserial;
pub mod pyshell;
pub mod pytee;

pub mod pyhook;

pub mod util;

pub mod pysdwirec;

pub mod pylogger;
pub mod pyasciicast;

use pyasciicast::PyAsciicast;
use pyexec::PyExec;
use pyhook::build_ttyhook;
use pylogger::{err, info, log, warn};
use pyo3::prelude::*;
use pysdwirec::PySdWirec;
use pyserial::PySerial;
use pyshell::PyShell;
use pytee::PyTee;
use shell_like::PyTty;
use util::{get_log_level, set_log_level};

#[pymodule]
#[pyo3(name = "tester")]
fn tester(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyTty>()?;
    m.add_class::<PyShell>()?;
    m.add_class::<PyTee>()?;
    m.add_class::<PyExec>()?;
    m.add_class::<PySerial>()?;
    m.add_class::<PySdWirec>()?;
    m.add_class::<PyAsciicast>()?;

    m.add_function(wrap_pyfunction!(build_ttyhook, m)?)?;
    m.add_function(wrap_pyfunction!(set_log_level, m)?)?;
    m.add_function(wrap_pyfunction!(get_log_level, m)?)?;

    m.add_function(wrap_pyfunction!(info, m)?)?;
    m.add_function(wrap_pyfunction!(log, m)?)?;
    m.add_function(wrap_pyfunction!(warn, m)?)?;
    m.add_function(wrap_pyfunction!(err, m)?)?;

    Ok(())
}
