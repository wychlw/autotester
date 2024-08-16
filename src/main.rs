use std::{fs, thread::sleep, time::Duration};

use tester::{
    consts::DURATION, exec::{
        cli_api::{CliTestApi, SudoCliTestApi},
        cli_exec::SudoCliTester,
    }, term::{
        asciicast::Asciicast,
        recorder::{Recorder, SimpleRecorder},
        shell::Shell,
        tty::Tty,
    }
};

fn main() {
    let shell = Shell::build(Some("/bin/sh")).unwrap();
    let mut shell = SimpleRecorder::build(shell);
    let _ = shell.begin();
    let mut shell = Asciicast::build(shell);

    let _ = shell.begin();

    shell
        .write(b"echo Hello, World!\n")
        .expect("Failed to write to stdin");
    loop {
        sleep(Duration::from_millis(DURATION));
        let data = shell.read_line().unwrap();
        let s = String::from_utf8(data).unwrap();
        if !s.is_empty() {
            println!("Recv1: {:#?}", s);
        }
        if s.contains("World") {
            break;
        }
    }
    shell.write(b"ls\n").expect("Failed to write to stdin");
    shell
        .write(b"sleep 1;echo After sleep\n")
        .expect("Failed to write to stdin");

    loop {
        sleep(Duration::from_millis(DURATION));
        let data = shell.read_line().unwrap();
        let data = String::from_utf8(data).unwrap();
        if !data.is_empty() {
            println!("Recv2: {:#?}", data);
        }
        if data.contains("After") {
            break;
        }
    }

    let mut exec = SudoCliTester::build(shell);

    let _ = exec.script_run("echo Hello, World!");

    let _ = exec.assert_script_run("echo \"Assert!\"", 2);

    let _ = exec.background_script_run("ls");

    let _ = exec.script_sudo("echo \"Sudo!\"");

    let _ = exec.assert_script_sudo("echo \"Assert Sudo!\"", 2);

    let mut shell = exec.exit();

    shell.write(b"exit\n").expect("Failed to write to stdin");

    println!("Done");

    let log = shell.end().unwrap();
    fs::write("test2.cast", log).expect("Failed to write to file");

    let mut shell = shell.exit();
    let log = shell.end().unwrap();
    fs::write("test2.log", log).expect("Failed to write to file");
    let shell = shell.exit();
    shell.stop();
}
