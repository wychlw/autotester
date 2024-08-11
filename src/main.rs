use std::{io::{BufReader, Read, Write}, process::{Command, Stdio}};

use tester::{logger::log, term::{shell::Shell, tty::Tty}};

fn main() {
    let mut shell = Shell::build(None).unwrap();

        shell.write(b"echo Hello, World!\n").expect("Failed to write to stdin");
        shell.write(b"ls\n").expect("Failed to write to stdin");
        shell.write(b"sleep 1; echo After sleep\n").expect("Failed to write to stdin");
        shell.write(b"exit\n").expect("Failed to write to stdin");

        loop {
            let data = shell.read().unwrap();
            if data.len() >= 1 {
                
            print!("Recv: {:#?}", String::from_utf8(data).unwrap());
            continue;
            }
            // break;
        }


    // loop {

    //     loop {
    //         let data = shell.read().unwrap();
    //         if data.len() < 1 {
    //             break;
    //         }
    //         print!("Recv: {:#?}", String::from_utf8(data).unwrap());
    //     }

    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input).unwrap();
    //     log(format!("Input: {}", input));
    //     shell.write(input.as_bytes()).unwrap();
    // }
}
