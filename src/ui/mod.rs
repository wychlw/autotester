//! Parts to handle the UI render part
//!
//! If only use CLI feats, this part can be ignored;
//! however, GUI part may need this part show what's going on.
//! Or, use to create needle for GUI part.
//!

// pub mod cli_hooker;
pub mod code_editor;
pub mod main;
pub mod pyenv;
pub mod terminal;
pub mod ui_cli_exec;
pub mod util;
pub mod ipc;

mod test_window;

use pyo3::{
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction, Bound, PyResult,
};
use ui_cli_exec::UiExec;
use util::__init_sub_virt__;

pub fn register_ui(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new_bound(parent_module.py(), "ui")?;
    m.add_class::<UiExec>()?;

    m.add_function(wrap_pyfunction!(__init_sub_virt__, &m)?)?;

    parent_module.add_submodule(&m)?;
    Ok(())
}
