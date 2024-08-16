use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use crate::{logger::err, term::tty::Tty, util::rand_string};

use super::cli_api::{CliTestApi, SudoCliTestApi};

pub struct CliTester<T>
where
    T: Tty,
{
    inner: T,
}

impl<T> CliTester<T>
where
    T: Tty,
{
    pub fn build(inner: T) -> CliTester<T> {
        CliTester { inner }
    }

    pub fn exit(self) -> T {
        self.inner
    }
}

impl<T> CliTester<T>
where
    T: Tty,
{
    fn run_command(&mut self, command: &String) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.inner.write(command.as_bytes());
        if let Err(e) = res {
            return Err(e);
        }
        Ok(())
    }
}

impl<T> CliTestApi for CliTester<T>
where
    T: Tty,
{
    fn script_run(&mut self, script: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = script.to_owned();
        cmd += "\n";
        self.run_command(&cmd)
    }
    fn assert_script_run(
        &mut self,
        script: &str,
        timeout: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = script.to_owned();
        let echo_content_rand = String::from_utf8(rand_string(8)).unwrap();

        cmd += "; echo ";
        cmd += &echo_content_rand;
        cmd += " \n";

        let begin = Instant::now();

        let res = self.run_command(&cmd);
        if let Err(e) = res {
            return Err(e);
        }

        let mut buf = Vec::new();
        loop {
            let res = self.inner.read();
            if let Err(e) = res {
                return Err(e);
            }
            let line = res.unwrap();
            buf.extend_from_slice(&line);
            let content = String::from_utf8(buf.clone()).unwrap();
            if content.contains(&echo_content_rand) {
                break;
            }
            if begin.elapsed().as_secs() > timeout as u64 {
                err(format!(
                    "Script timeout! {}",
                    String::from_utf8(buf.clone()).unwrap()
                ));
                return Err(Box::<dyn std::error::Error>::from("Timeout"));
            }
            sleep(Duration::from_millis(100));
        }
        Ok(())
    }
    fn background_script_run(&mut self, script: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = script.to_owned();
        cmd += " &\n";
        self.run_command(&cmd)
    }
}

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

    pub fn exit(self) -> T {
        self.inner.exit()
    }
}

impl<T> CliTestApi for SudoCliTester<T>
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

impl<T> SudoCliTestApi for SudoCliTester<T>
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
