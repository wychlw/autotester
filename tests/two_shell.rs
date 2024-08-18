use std::fs;

use tester::{
    devhost::sdwirec::{SdwirecChooser, SdwirecProd},
    exec::{
        cli_api::{CliTestApi, ExecBase, SudoCliTestApi},
        cli_exec_sudo::SudoCliTester,
    },
    term::{
        asciicast::Asciicast,
        recorder::{Recorder, SimpleRecorder},
        shell::Shell,
        tty::WrapperTty,
    },
};

#[test]
fn two_shell() {
    let sd = SdwirecProd::new(SdwirecChooser::Id(0));

    let ts = Shell::build(Some("/bin/sh")).unwrap();
    // let dut = Serial::build("/dev/ttyUSB0", 115200).unwrap();
    let dut = Shell::build(Some("/bin/sh")).unwrap();
    let mut ts = SimpleRecorder::build(ts);
    ts.begin().unwrap();
    let mut dut = SimpleRecorder::build(dut);
    dut.begin().unwrap();
    let mut rec =  Asciicast::build(ts);
    rec.begin().unwrap();


    let mut exec = SudoCliTester::build(rec);

    // exec.script_run("tty").unwrap();
    exec.script_run("ls").unwrap();

    exec.assert_script_run("mkdir /tmp/test1", 5).unwrap();
    exec.assert_script_run("echo \"Test Test\" > /tmp/test1/test.txt", 5).unwrap();

    let mut ts = exec.inner_mut().swap(dut).unwrap(); // TODO: 动态多态支持，实现运行时不同类切换 log 记录设备，顺手动态切换 Exec 后台

    exec.assert_script_run("sleep 1", 5).unwrap();
    // exec.script_run("tty").unwrap();
    exec.assert_script_sudo("cat /tmp/test1/test.txt", 5).unwrap();

    println!("Done");

    let mut rec = exec.exit();

    let rec_log = rec.end().unwrap();
    let mut dut = rec.exit();
    let ts_log = ts.end().unwrap();
    let dut_log = dut.end().unwrap();
    
    let ts=ts.exit();
    let dut=dut.exit();

    fs::write("ts.log", ts_log).unwrap();
    fs::write("dut.log", dut_log).unwrap();
    fs::write("rec.cast", rec_log).unwrap();

    ts.stop();
    dut.stop();
}
