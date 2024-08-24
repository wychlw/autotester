use std::{
    any::Any,
    thread::sleep,
    time::{Duration, Instant},
};

use crate::{
    consts::DURATION,
    logger::err,
    term::tty::{DynTty, Tty, WrapperTty},
    util::{anybase::AnyBase, util::rand_string},
};

use super::cli_api::{CliTestApi, ExecBase};

pub struct CliTester {
    inner: DynTty,
}

impl CliTester {
    pub fn build(inner: DynTty) -> CliTester {
        CliTester { inner }
    }
}

impl CliTester {
    fn run_command(&mut self, command: &String) -> Result<(), Box<dyn std::error::Error>> {
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

impl WrapperTty for CliTester {
    fn exit(self) -> DynTty {
        self.inner
    }
}

impl ExecBase for CliTester {
    fn inner_ref(&self) -> &DynTty {
        &self.inner
    }
    fn inner_mut(&mut self) -> &mut DynTty {
        &mut self.inner
    }
}

impl CliTestApi for CliTester {
    fn script_run(&mut self, script: &str, timeout: u32) -> Result<(), Box<dyn std::error::Error>> {
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
                return Ok(());
            }
        }
        Ok(())
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
    fn writeln(&mut self, script: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = script.to_owned();
        cmd += "\n";
        self.run_command(&cmd)
    }
}
