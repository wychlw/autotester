pub mod shell_like;

pub mod exec;
pub mod serial;
pub mod shell;
pub mod tee;

pub mod hook;

pub mod util;

pub mod sdwirec;

mod pylogger;
pub mod asciicast;

pub mod deansi;

use deansi::DeANSI;
use asciicast::Asciicast;
use exec::Exec;
use hook::build_ttyhook;
use pylogger::{err, info, log, warn};
use pyo3::prelude::*;
use sdwirec::SdWirec;
use serial::Serial;
use shell::Shell;
use tee::Tee;
use shell_like::PyTty;
use util::{get_log_level, run_ui, set_log_level};

use crate::ui::register_ui;

#[pymodule]
#[pyo3(name = "tester")]
fn tester(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyTty>()?;
    m.add_class::<Shell>()?;
    m.add_class::<Tee>()?;
    m.add_class::<Exec>()?;
    m.add_class::<Serial>()?;
    m.add_class::<SdWirec>()?;
    m.add_class::<Asciicast>()?;
    m.add_class::<DeANSI>()?;

    m.add_function(wrap_pyfunction!(build_ttyhook, m)?)?;
    m.add_function(wrap_pyfunction!(set_log_level, m)?)?;
    m.add_function(wrap_pyfunction!(get_log_level, m)?)?;

    m.add_function(wrap_pyfunction!(run_ui, m)?)?;

    m.add_function(wrap_pyfunction!(info, m)?)?;
    m.add_function(wrap_pyfunction!(log, m)?)?;
    m.add_function(wrap_pyfunction!(warn, m)?)?;
    m.add_function(wrap_pyfunction!(err, m)?)?;

    register_ui(m)?;

    Ok(())
}
