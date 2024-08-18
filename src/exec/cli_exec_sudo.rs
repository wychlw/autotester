use crate::term::tty::{Tty, WrapperTty};

use super::{cli_api::{CliTestApi, ExecBase, SudoCliTestApi}, cli_exec::CliTester};

pub struct SudoCliTester<T>
where
    T: Tty,
{
    inner: CliTester<T>,
}

impl<T> SudoCliTester<T>
where
    T: Tty,
{
    pub fn build(inner: T) -> SudoCliTester<T> {
        SudoCliTester {
            inner: CliTester::build(inner),
        }
    }
}

impl<T> Tty for SudoCliTester<T>
where
    T: Tty,
{
    // Note: This will SKIP the logic in the tester
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.inner.read()
    }
    // Note: This will SKIP the logic in the tester
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.inner.read_line()
    }
    // Note: This will SKIP the logic in the tester
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.inner.write(data)
    }
}


impl<T> WrapperTty<T> for SudoCliTester<T>
where
    T: Tty,
{
    fn exit(self) -> T {
        self.inner.exit()
    }
}

impl<T> ExecBase<T> for SudoCliTester<T>
where
    T: Tty,
{
    fn inner_ref(&self) -> &T {
        self.inner.inner_ref()
    }
    fn inner_mut(&mut self) -> &mut T {
        self.inner.inner_mut()
    }
}

impl<T> CliTestApi<T> for SudoCliTester<T>
where
    T: Tty,
{
    fn script_run(&mut self, script: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.inner.script_run(script)
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
}

impl<T> SudoCliTestApi<T> for SudoCliTester<T>
where
    T: Tty,
{
    fn script_sudo(&mut self, script: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = String::from("sudo ");
        cmd += script;
        cmd += "\n";
        self.inner.script_run(&cmd)
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

