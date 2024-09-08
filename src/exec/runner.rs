use super::cli_api::SudoCliTestApi;

pub struct Cmd {
    cmd: String,
}

pub struct TimeoutCmd {
    cmd: String,
    timeout: u32,
}

pub struct WaitCmd {
    cmd: String,
    wait: String,
    timeout: u32,
}

pub enum CmdType {
    Direct(Cmd),               // writeln(cmd)
    Run(TimeoutCmd),           // script_run(cmd, timeout)
    AssertRun(TimeoutCmd),     // assert_script_run(cmd, timeout)
    SudoRun(TimeoutCmd),       // script_sudo(cmd, timeout)
    SudoAssertRun(TimeoutCmd), // assert_script_sudo(cmd, timeout)
    Wait(TimeoutCmd),          // wait_serial(cmd, timeout)
    WaitRun(WaitCmd),          // writeln(cmd) wait_serial(wait, timeout)
}

pub struct CmdRunner {
    cmds: Vec<CmdType>,
    inner: Box<dyn SudoCliTestApi>,
}

impl CmdRunner {
    pub fn build(inner: Box<dyn SudoCliTestApi>, cmds: Vec<CmdType>) -> Self {
        Self { cmds, inner }
    }
}

impl CmdRunner {
    pub fn inner_ref(&self) -> &Box<dyn SudoCliTestApi> {
        &self.inner
    }
    pub fn inner_mut(&mut self) -> &mut Box<dyn SudoCliTestApi> {
        &mut self.inner
    }
}

impl CmdRunner {
    pub fn renew(&mut self, cmds: Vec<CmdType>) {
        self.cmds = cmds;
    }
    pub fn run(&mut self) {
        for cmd in self.cmds.iter() {
            match cmd {
                CmdType::Direct(cmd) => {
                    self.inner.writeln(&cmd.cmd).unwrap();
                }
                CmdType::Run(cmd) => {
                    self.inner.script_run(&cmd.cmd, cmd.timeout).unwrap();
                }
                CmdType::AssertRun(cmd) => {
                    self.inner.assert_script_run(&cmd.cmd, cmd.timeout).unwrap();
                }
                CmdType::SudoRun(cmd) => {
                    self.inner.script_sudo(&cmd.cmd, cmd.timeout).unwrap();
                }
                CmdType::SudoAssertRun(cmd) => {
                    self.inner
                        .assert_script_sudo(&cmd.cmd, cmd.timeout)
                        .unwrap();
                }
                CmdType::Wait(cmd) => {
                    self.inner.wait_serial(&cmd.cmd, cmd.timeout).unwrap();
                }
                CmdType::WaitRun(cmd) => {
                    self.inner.writeln(&cmd.cmd).unwrap();
                    self.inner.wait_serial(&cmd.wait, cmd.timeout).unwrap();
                }
            }
        }
    }
}
