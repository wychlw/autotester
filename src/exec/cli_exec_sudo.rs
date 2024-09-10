use std::any::Any;

use crate::{
    term::tty::{DynTty, InnerTty, Tty, WrapperTty},
    util::anybase::AnyBase,
};

use super::{
    cli_api::{CliTestApi, SudoCliTestApi},
    cli_exec::CliTester,
};

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
    fn wait_serial(&mut self, expected: &str, timeout: u32) -> Result<(), Box<dyn std::error::Error>> {
        self.inner.wait_serial(expected, timeout)
    }
    fn script_run(&mut self, script: &str, timeout: u32) -> Result<(), Box<dyn std::error::Error>> {
        self.inner.script_run(script, timeout)
    }
    fn assert_script_run(
        &mut self,
        script: &str,
        timeout: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.inner.assert_script_run(script, timeout)
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
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = String::from("sudo ");
        cmd += script;
        cmd += "\n";
        self.inner.script_run(&cmd, timeout)
    }
    fn assert_script_sudo(
        &mut self,
        script: &str,
        timeout: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = String::from("sudo ");
        cmd += script;
        self.inner.assert_script_run(&cmd, timeout)
    }
}
