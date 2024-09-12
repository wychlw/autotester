use std::{
    any::Any,
    error::Error,
    thread::sleep,
    time::{Duration, Instant},
};

use crate::{
    consts::DURATION, err, info, log, term::tty::{DynTty, InnerTty, Tty, WrapperTty}, util::{anybase::AnyBase, util::rand_string}
};

use super::cli_api::CliTestApi;

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
        let res = self.inner.write(command.as_bytes());
        if let Err(e) = res {
            return Err(e);
        }
        Ok(())
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
    fn wait_serial(&mut self, expected: &str, timeout: u32) -> Result<(), Box<dyn Error>> {
        let begin = Instant::now();
        let mut buf = Vec::new();
        info!("Waiting for string {{{}}}", expected);
        loop {
            sleep(Duration::from_millis(DURATION));
            let res = self.inner.read();
            if let Err(e) = res {
                return Err(e);
            }
            let line = res.unwrap();
            buf.extend_from_slice(&line);
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

        Ok(())
    }
    fn script_run(&mut self, script: &str, timeout: u32) -> Result<(), Box<dyn Error>> {
        let mut cmd = script.to_owned();
        let echo_content_rand = String::from_utf8(rand_string(8)).unwrap();

        cmd += "&& echo ";
        cmd += &echo_content_rand;
        cmd += " \n";

        let res = self.run_command(&cmd);
        if let Err(e) = res {
            return Err(e);
        }

        let res = self.wait_serial(&echo_content_rand, timeout);
        if let Err(e) = res {
            if e.to_string() == "Timeout" {
                return Ok(());
            }
            return Err(e);
        }
        res
    }
    fn assert_script_run(&mut self, script: &str, timeout: u32) -> Result<(), Box<dyn Error>> {
        let mut cmd = script.to_owned();
        let echo_content_rand = String::from_utf8(rand_string(8)).unwrap();

        cmd += "&& echo ";
        cmd += &echo_content_rand;
        cmd += " \n";

        let res = self.run_command(&cmd);
        if let Err(e) = res {
            return Err(e);
        }

        let res = self.wait_serial(&echo_content_rand, timeout);
        res
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
