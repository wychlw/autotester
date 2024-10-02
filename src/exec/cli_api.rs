//! The API for the CLI test.
use std::error::Error;

use crate::cli::tty::WrapperTty;

/// The API for the CLI test.
pub trait CliTestApi: WrapperTty {
    /// Run a script in the terminal, wait for the script to finish(or timeout), and return the output.
    /// 
    /// You may found this func includes assert_script_run and script_output 
    fn script_run(&mut self, script: &str, timeout: u32) -> Result<String, Box<dyn Error>>;

    /// Run a script in the terminal which is a background command
    fn background_script_run(&mut self, script: &str) -> Result<(), Box<dyn Error>>;

    /// Write someting to the terminal.
    fn writeln(&mut self, script: &str) -> Result<(), Box<dyn Error>>;

    /// Wait for the terminal to output the expected string. Output the terminal output when the expected string is found.
    fn wait_serial(&mut self, expected: &str, timeout: u32) -> Result<String, Box<dyn Error>>;
}

pub trait SudoCliTestApi: CliTestApi {
    /// Just like script_run, but with sudo.
    /// 
    /// You may found this func includes assert_script_sudo and script_output
    fn script_sudo(&mut self, script: &str, timeout: u32) -> Result<String, Box<dyn Error>>;
}
