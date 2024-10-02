//! How to execute some operations for the CLI and GUI.
//! 
//! This part contains the interactive API for the CLI and GUI. You definitely 
//! could use the CLI API and GUI API directly, but you really want to see a bunch of
//! blocking IO and u8 slices?
//! So these wrappers are here to provide high-level API for the CLI and GUI.

pub mod cli_api;
pub mod cli_exec;
pub mod gui_api;
pub mod needle;
pub mod gui_exec;

pub mod gui_handler;