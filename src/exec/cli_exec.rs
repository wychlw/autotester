use std::{
    any::Any,
    error::Error,
    thread::sleep,
    time::{Duration, Instant},
};

use crate::{
    consts::DURATION, err, info, cli::tty::{DynTty, InnerTty, Tty, WrapperTty}, util::{anybase::AnyBase, util::rand_string}
};

use super::cli_api::{CliTestApi, SudoCliTestApi};

pub struct CliTester {
    inner: DynTty,
}

impl CliTester {
    pub fn build(inner: DynTty) -> CliTester {
        CliTester { inner }
    }
}

impl CliTester {
    fn run_command(&mut self, command: &String) -> Result<(), Box<dyn Error>> {
        info!("Write to shell: {}", command);
        self.inner.write(command.as_bytes())
    }
}

impl AnyBase for CliTester {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Tty for CliTester {
    // Note: This will SKIP the logic in the tester
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        self.inner.read()
    }
    // Note: This will SKIP the logic in the tester
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        self.inner.read_line()
    }
    // Note: This will SKIP the logic in the tester
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        self.inner.write(data)
    }
}

impl WrapperTty for CliTester {
    fn exit(self) -> DynTty {
        self.inner
    }
}

impl InnerTty for CliTester {
    fn inner_ref(&self) -> &DynTty {
        &self.inner
    }
    fn inner_mut(&mut self) -> &mut DynTty {
        &mut self.inner
    }
}

impl CliTestApi for CliTester {
    fn wait_serial(&mut self, expected: &str, timeout: u32) -> Result<String, Box<dyn Error>> {
        let begin = Instant::now();
        let mut buf = Vec::new();
        info!("Waiting for string {{{}}}", expected);
        loop {
            sleep(Duration::from_millis(DURATION));
            let res = self.inner.read()?;
            buf.extend_from_slice(&res);
            let content = String::from_utf8(buf.clone()).unwrap_or_default();
            if content.contains(expected) {
                info!("Matched string {{{}}}", expected);
                break;
            }
            if begin.elapsed().as_secs() > timeout as u64 {
                err!(
                    "Timeout! Expected: {}, Actual: {}",
                    expected,
                    String::from_utf8(buf.clone()).unwrap()
                );
                return Err(Box::<dyn Error>::from("Timeout"));
            }
        }

        let res = String::from_utf8(buf)?;
        Ok(res)
    }
    fn script_run(&mut self, script: &str, timeout: u32) -> Result<String, Box<dyn Error>> {
        let mut cmd = script.to_owned();
        let echo_content_rand = String::from_utf8(rand_string(8)).unwrap();

        cmd += "&& echo ";
        cmd += &echo_content_rand;
        cmd += " \n";

        self.run_command(&cmd)?;

        self.wait_serial(&echo_content_rand, timeout)
    }
    fn background_script_run(&mut self, script: &str) -> Result<(), Box<dyn Error>> {
        let mut cmd = script.to_owned();
        cmd += " &\n";
        self.run_command(&cmd)
    }
    fn writeln(&mut self, script: &str) -> Result<(), Box<dyn Error>> {
        let mut cmd = script.to_owned();
        cmd += "\n";
        self.run_command(&cmd)
    }
}

pub struct SudoCliTester {
    inner: CliTester,
}

impl SudoCliTester {
    pub fn build(inner: DynTty) -> SudoCliTester {
        SudoCliTester {
            inner: CliTester::build(inner),
        }
    }
}

impl AnyBase for SudoCliTester {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Tty for SudoCliTester {
    // Note: This will SKIP the logic in the tester
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.inner.read()
    }
    // Note: This will SKIP the logic in the tester
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.inner.read_line()
    }
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.inner.write(data)
    }
}

impl WrapperTty for SudoCliTester {
    fn exit(self) -> DynTty {
        self.inner.exit()
    }
}

impl InnerTty for SudoCliTester {
    fn inner_ref(&self) -> &DynTty {
        self.inner.inner_ref()
    }
    fn inner_mut(&mut self) -> &mut DynTty {
        self.inner.inner_mut()
    }
}

impl CliTestApi for SudoCliTester {
    fn wait_serial(&mut self, expected: &str, timeout: u32) -> Result<String, Box<dyn std::error::Error>> {
        self.inner.wait_serial(expected, timeout)
    }
    fn script_run(&mut self, script: &str, timeout: u32) -> Result<String, Box<dyn std::error::Error>> {
        self.inner.script_run(script, timeout)
    }
    fn background_script_run(&mut self, script: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.inner.background_script_run(script)
    }
    fn writeln(&mut self, script: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.inner.writeln(script)
    }
}

impl SudoCliTestApi for SudoCliTester {
    fn script_sudo(
        &mut self,
        script: &str,
        timeout: u32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut cmd = String::from("sudo ");
        cmd += script;
        cmd += "\n";
        self.inner.script_run(&cmd, timeout)
    }
}
