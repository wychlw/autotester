use std::error::Error;

use crate::cli::tty::InnerTty;

pub trait CliTestApi: InnerTty {
    /// 
    /// 
    /// You may found this func includes assert_script_run and script_output 
    fn script_run(&mut self, script: &str, timeout: u32) -> Result<String, Box<dyn Error>>;
    fn background_script_run(&mut self, script: &str) -> Result<(), Box<dyn Error>>;
    fn writeln(&mut self, script: &str) -> Result<(), Box<dyn Error>>;
    fn wait_serial(&mut self, expected: &str, timeout: u32) -> Result<String, Box<dyn Error>>;
}

pub trait SudoCliTestApi: CliTestApi {
    ///
    /// 
    /// You may found this func includes assert_script_sudo and script_output
    fn script_sudo(&mut self, script: &str, timeout: u32) -> Result<String, Box<dyn Error>>;
}
