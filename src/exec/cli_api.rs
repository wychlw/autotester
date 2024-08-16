use std::error::Error;

pub trait CliTestApi {
    fn script_run(&mut self, script: &str) -> Result<(), Box<dyn Error>>;
    fn assert_script_run(&mut self, script: &str, timeout: u32) -> Result<(), Box<dyn Error>>;
    fn background_script_run(&mut self, script: &str) -> Result<(), Box<dyn Error>>;
    // fn script_output(&mut self, script: &str) -> Result<Vec<u8>, Box<dyn Error>>;
    // fn validate_script_output(&mut self, script: &str, expected_output: &str) -> Result<(), Box<dyn Error>>;
}

pub trait SudoCliTestApi {
    fn script_sudo(&mut self, script: &str) -> Result<(), Box<dyn Error>>;
    fn assert_script_sudo(&mut self, script: &str, timeout: u32) -> Result<(), Box<dyn Error>>;
}
