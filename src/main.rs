
use tester::term::{shell::Shell, tty::Tty};

fn main() {
    let mut shell = Shell::build(None).unwrap();

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
        .write(b"sleep 1; echo After sleep\n")
        .expect("Failed to write to stdin");
    shell.write(b"exit\n").expect("Failed to write to stdin");

    loop {
        let data = shell.read_line().unwrap();
        if !data.is_empty() {
            print!("Recv2: {:#?}", String::from_utf8(data).unwrap());
            continue;
        }
        // break;
    }
}
