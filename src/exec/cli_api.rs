use std::error::Error;

pub trait CliTestApi {
    fn script_run(&mut self, script: &[u8]) -> Result<(), Box<dyn Error>>;
    fn assert_script_run(&mut self, script: &[u8], timeout: u32) -> Result<(), Box<dyn Error>>;
    fn background_script_run(&mut self, script: &[u8]) -> Result<(), Box<dyn Error>>;
    // fn script_sudo(&mut self, script: &[u8]) -> Result<(), Box<dyn Error>>;
    // fn assert_script_sudo(&mut self, script: &[u8]) -> Result<(), Box<dyn Error>>;
    // fn script_output(&mut self, script: &[u8]) -> Result<Vec<u8>, Box<dyn Error>>;
    // fn validate_script_output(&mut self, script: &[u8], expected_output: &[u8]) -> Result<(), Box<dyn Error>>;
}
