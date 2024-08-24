use std::fs;

use tester::{
    dyn_cast_mut, dyn_into,
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
    let ts = Shell::build(Some("/bin/sh")).unwrap();
    // let dut = Serial::build("/dev/ttyUSB0", 115200).unwrap();
    let dut = Shell::build(Some("/bin/sh")).unwrap();
    let mut ts = SimpleRecorder::build(Box::new(ts));
    ts.begin().unwrap();
    let mut dut = SimpleRecorder::build(Box::new(dut));
    dut.begin().unwrap();
    let mut rec = Asciicast::build(Box::new(ts));
    rec.begin().unwrap();

    let mut exec = SudoCliTester::build(Box::new(rec));

    // exec.script_run("tty").unwrap();
    exec.script_run("ls", 2).unwrap();

    exec.assert_script_run("mkdir /tmp/test1", 5).unwrap();
    exec.assert_script_run("echo \"Test Test\" > /tmp/test1/test.txt", 5)
        .unwrap();

    let rec = dyn_cast_mut!(exec.inner_mut(), Asciicast).unwrap();
    let ts = rec.swap(Box::new(dut)).unwrap();

    exec.assert_script_run("sleep 1", 5).unwrap();
    // exec.script_run("tty").unwrap();
    exec.assert_script_sudo("cat /tmp/test1/test.txt", 5)
        .unwrap();
    exec.writeln("date").unwrap();
    exec.script_run("sleep 1", 2).unwrap();

    println!("Done");

    let rec = exec.exit();
    let mut rec = dyn_into!(rec, Asciicast).unwrap();

    let rec_log = rec.end().unwrap();
    let dut = rec.exit();
    let mut dut = dyn_into!(dut, SimpleRecorder).unwrap();
    let mut ts = dyn_into!(ts, SimpleRecorder).unwrap();
    let ts_log = ts.end().unwrap();
    let dut_log = dut.end().unwrap();

    let ts = ts.exit();
    let ts = dyn_into!(ts, Shell).unwrap();
    let dut = dut.exit();
    let dut = dyn_into!(dut, Shell).unwrap();

    fs::write("ts.log", ts_log).unwrap();
    fs::write("dut.log", dut_log).unwrap();
    fs::write("rec.cast", rec_log).unwrap();

    ts.stop();
    dut.stop();
}
