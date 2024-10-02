//! Parts to handle the UI render part
//! 
//! If only use CLI feats, this part can be ignored;
//! however, GUI part may need this part show what's going on.
//! Or, use to create needle for GUI part.
//! 

pub mod main;
pub mod code_editor;
pub mod pyenv;
pub mod terminal;
pub mod cli_hooker;
pub mod ui_cli_exec;
pub mod util;

use pyo3::{types::{PyModule, PyModuleMethods}, Bound, PyResult};
use ui_cli_exec::UiExec;

pub fn register_ui(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new_bound(parent_module.py(), "ui")?;
    m.add_class::<UiExec>()?;

    parent_module.add_submodule(&m)?;
    Ok(())
}
