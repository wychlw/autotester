use std::{thread::sleep, time::{Duration, Instant}};

use crate::{logger::err, term::tty::Tty, util::rand_string};

use super::cli_api::CliTestApi;

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
    fn run_command(&mut self, command: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.inner.write(&command);
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
    fn script_run(&mut self, script: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = script.to_owned();
        cmd.push(b'\n');
        self.run_command(&cmd)
    }
    fn assert_script_run(
        &mut self,
        script: &[u8],
        timeout: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = script.to_owned();
        cmd.push(b';');
        cmd.push(b' ');
        let echo_content_rand = rand_string(8);

        cmd.extend_from_slice(b"echo ");
        cmd.extend_from_slice(&echo_content_rand);
        cmd.push(b';');
        cmd.push(b'\n');

        let begin = Instant::now();

        let res = self.run_command(&cmd);
        if let Err(e) = res {
            return Err(e);
        }

        let mut buf = Vec::new();
        let echo_content_rand = String::from_utf8(echo_content_rand).unwrap();
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
    fn background_script_run(
        &mut self,
        script: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = script.to_owned();
        cmd.push(b' ');
        cmd.push(b'&');
        cmd.push(b'\n');
        self.run_command(&cmd)
    }
}
