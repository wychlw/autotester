
use std::fs;

use tester::{exec::{cli_api::CliTestApi, cli_exec::CliTester}, term::{asciicast::Asciicast, recorder::Recorder, shell::Shell, tty::Tty}};

fn main() {
    let shell = Shell::build(Some("/bin/sh")).unwrap();
    let mut shell = Asciicast::build(shell);

    let _ = shell.begin();

    shell
        .write(b"echo Hello, World!\n")
        .expect("Failed to write to stdin");
    loop {
        let data = shell.read_line().unwrap();
        let s = String::from_utf8(data).unwrap();
        if !s.is_empty() {
            print!("Recv1: {:#?}", s);
        }
        if s.contains("World") {
            break;
        }
    }
    shell.write(b"ls\n").expect("Failed to write to stdin");
    shell
        .write(b"echo After sleep\n")
        .expect("Failed to write to stdin");
    // shell.write(b"exit\n").expect("Failed to write to stdin");

    loop {
        let data = shell.read_line().unwrap();
        let data = String::from_utf8(data).unwrap();
        if !data.is_empty() {
            println!("Recv2: {:#?}", data);
        }
        if data.contains("After") {
            break;
        }
    }

    let mut exec = CliTester::build(shell);

    let _ = exec.script_run(b"echo Hello, World!");

    let _ = exec.assert_script_run(b"echo \"Assert!\"", 2);

    let _ = exec.background_script_run(b"ls");

    let mut shell = exec.exit();

    shell.write(b"exit\n").expect("Failed to write to stdin");

    println!("Done");

    let log = shell.end().unwrap();
    fs::write("test2.cast", log).expect("Failed to write to file");

    let mut shell = shell.exit();
    shell.stop();

}
