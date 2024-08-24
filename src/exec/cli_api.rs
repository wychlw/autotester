use std::error::Error;

use crate::term::tty::{DynTty, WrapperTty};

pub trait ExecBase: WrapperTty {
    fn inner_ref(&self) -> &DynTty;
    fn inner_mut(&mut self) -> &mut DynTty;
}

pub trait CliTestApi: ExecBase {
    fn script_run(&mut self, script: &str, timeout: u32) -> Result<(), Box<dyn Error>>;
    fn assert_script_run(&mut self, script: &str, timeout: u32) -> Result<(), Box<dyn Error>>;
    fn background_script_run(&mut self, script: &str) -> Result<(), Box<dyn Error>>;
    fn writeln(&mut self, script: &str) -> Result<(), Box<dyn Error>>;
    // fn script_output(&mut self, script: &str) -> Result<Vec<u8>, Box<dyn Error>>;
    // fn validate_script_output(&mut self, script: &str, expected_output: &str) -> Result<(), Box<dyn Error>>;
}

pub trait SudoCliTestApi: CliTestApi {
    fn script_sudo(&mut self, script: &str, timeout: u32) -> Result<(), Box<dyn Error>>;
    fn assert_script_sudo(&mut self, script: &str, timeout: u32) -> Result<(), Box<dyn Error>>;
}
