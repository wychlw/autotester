use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use crate::{consts::DURATION, logger::err, term::tty::{Tty, WrapperTty}, util::rand_string};

use super::cli_api::{CliTestApi, ExecBase};

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

impl<T> Tty for CliTester<T>
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

impl<T> WrapperTty<T> for CliTester<T>
where
    T: Tty,
{
    fn exit(self) -> T {
        self.inner
    }
}

impl<T> ExecBase<T> for CliTester<T>
where
    T: Tty,
{
    fn inner_ref(&self) -> &T {
        &self.inner
    }
    fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T> CliTestApi<T> for CliTester<T>
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
            sleep(Duration::from_millis(DURATION));
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
        }
        Ok(())
    }
    fn background_script_run(&mut self, script: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = script.to_owned();
        cmd += " &\n";
        self.run_command(&cmd)
    }
}

