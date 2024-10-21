//! The implementation of the CLI tester. Look at [`CliTestApi`] for more information.

use std::{
    error::Error,
    thread::sleep,
    time::{Duration, Instant},
};

use crate::{
    cli::tty::{DynTty, Tty, WrapperTty},
    consts::DURATION,
    err, impl_any, info,
    util::util::rand_string,
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
        sleep(Duration::from_millis(DURATION));
        self.inner.write(command.as_bytes())
    }
}

impl_any!(CliTester);

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

    fn inner_ref(&self) -> &DynTty {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut DynTty {
        &mut self.inner
    }
}

impl CliTester {
    fn filter_assert_echo(&self, expected: &str, buf: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
        let expected = "echo ".to_owned() + expected;
        let expected = expected.as_bytes();
        for (pos, window) in buf.windows(expected.len()).enumerate() {
            if window == expected {
                let i = pos + expected.len();
                buf.drain(0..=i);
                break;
            }
        }
        Ok(())
    }

    fn kmp_next(&self, target: &Vec<u8>) -> Vec<usize> {
        let mut next = vec![0usize; target.len()];
        let mut i = 1;
        let mut j = 0;
        while i < target.len() - 1 {
            if target[i] == target[j] {
                next[i] = j + 1;
                i += 1;
                j += 1;
            } else {
                if j == 0 {
                    next[i] = 0;
                    i += 1;
                } else {
                    j = next[j - 1] as usize;
                }
            }
        }
        next
    }

    fn kmp_search(&self, content: &Vec<u8>, target: &Vec<u8>) -> Option<usize> {
        let next = self.kmp_next(target);
        let mut i = 0;
        let mut j = 0;
        let mut res = None;
        while i < content.len() && j < target.len() {
            if content[i] == target[j] {
                if res.is_none() {
                    res = Some(i);
                }
                i += 1;
                j += 1;
                if j >= target.len() {
                    break;
                }
            } else {
                if j == 0 {
                    i += 1;
                } else {
                    j = next[j - 1];
                }
                res = None;
            }
        }
        res
    }

    fn do_wait_serial(
        &mut self,
        expected: &str,
        timeout: u32,
        filter_echo_back: Option<&str>,
    ) -> Result<String, Box<dyn Error>> {
        let begin = Instant::now();
        let mut buf = Vec::new();
        info!("Waiting for string {{{}}}", expected);
        loop {
            sleep(Duration::from_millis(DURATION));
            let res = self.inner.read()?;
            buf.extend_from_slice(&res);
            if let Some(filter) = filter_echo_back {
                self.filter_assert_echo(filter, &mut buf)?;
            }
            // The reason we compare raw u8 is... What if the data is corrupted?
            let target = expected.as_bytes();
            if let Some(pos) = self.kmp_search(&buf, &target.to_vec()) {
                info!("Matched string {{{}}}", expected);
                let res = buf.split_off(pos + target.len());
                let res = String::from_utf8(res)?;
                buf.drain(0..pos + target.len());
                return Ok(res);
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
    }
}

impl CliTestApi for CliTester {
    fn wait_serial(&mut self, expected: &str, timeout: u32) -> Result<String, Box<dyn Error>> {
        self.do_wait_serial(expected, timeout, None)
    }
    fn script_run(&mut self, script: &str, timeout: u32) -> Result<String, Box<dyn Error>> {
        let mut cmd = script.to_owned();
        let echo_content_rand = rand_string(8);

        cmd += " && echo ";
        cmd += &echo_content_rand;
        cmd += " \n";

        self.run_command(&cmd)?;

        self.do_wait_serial(&echo_content_rand, timeout, Some(&echo_content_rand))
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

impl_any!(SudoCliTester);

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

    fn inner_ref(&self) -> &DynTty {
        self.inner.inner_ref()
    }

    fn inner_mut(&mut self) -> &mut DynTty {
        self.inner.inner_mut()
    }
}

impl CliTestApi for SudoCliTester {
    fn wait_serial(
        &mut self,
        expected: &str,
        timeout: u32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.inner.wait_serial(expected, timeout)
    }
    fn script_run(
        &mut self,
        script: &str,
        timeout: u32,
    ) -> Result<String, Box<dyn std::error::Error>> {
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
        cmd += " ";
        self.inner.script_run(&cmd, timeout)
    }
}
