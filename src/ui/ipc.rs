//! IPC between windows and sub-windows.
//!
//! As the program may run in multiple processes, or in same process but different address space,
//! we need a way to communicate between windows and sub-windows.

use std::{
    error::Error,
    io::{BufRead, BufReader, ErrorKind, Write},
};

use interprocess::local_socket::{
    prelude::*, traits::ListenerExt, GenericNamespaced, Listener, ListenerNonblockingMode,
    ListenerOptions, Stream, ToNsName,
};
use serde::{Deserialize, Serialize};

use crate::{err, log, ui::util::get_sub_virt};

use super::{main::MyApp, util::get_main_virt};

#[derive(Serialize, Deserialize)]
pub struct WindowIpcMessage {
    pub window_id: u64,
    pub message: String,
}

pub fn init_ipc() -> Result<Listener, Box<dyn Error>> {
    let sock_name = get_sock_name(get_main_virt().to_owned(), None);
    let sock_name = sock_name.to_ns_name::<GenericNamespaced>()?;

    let opts = ListenerOptions::new()
        .name(sock_name)
        .nonblocking(ListenerNonblockingMode::Both);

    let listener = opts.create_sync()?;
    Ok(listener)
}

impl MyApp {
    pub(super) fn handle_ipc(&mut self) {
        for m in self.listener.incoming() {
            match m {
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    break;
                }
                Err(e) => {
                    err!("IPC error: {}", e);
                    continue;
                }
                Ok(mut stream) => {
                    log!("IPC message received");
                    let mut reader = BufReader::new(&mut stream);
                    let mut buf = String::new();
                    let _ = reader.read_line(&mut buf);
                    log!("Received IPC message: {}", buf);
                    // let msg: WindowIpcMessage = match from_reader(&mut reader) {
                    let msg: WindowIpcMessage = match serde_json::from_str(&buf) {
                        Ok(m) => m,
                        Err(e) => {
                            err!("IPC message decode error: {}", e);
                            continue;
                        }
                    };
                    log!(
                        "Received IPC message from window {}: {}",
                        msg.window_id,
                        msg.message
                    );
                    for w in self.sub_windows.iter_mut() {
                        if w.idx == msg.window_id {
                            w.window.on_ipc(&msg.message, &mut stream);
                        }
                    }
                }
            }
        }
    }
}

pub fn get_sock_name(base_name: String, window_id: Option<u64>) -> String {
    log!(
        "Try get sock name with base name: {}, window id: {:?}",
        base_name,
        window_id
    );
    if let Some(id) = window_id {
        format!("{}_{}.sock", base_name, id)
    } else {
        base_name + ".sock"
    }
}

pub fn parse_sock_id(sock_name: &str) -> Option<u64> {
    log!("Try parse sock id from name: {}", sock_name);
    let parts = sock_name.split('_');
    let mut id = parts.rev().next()?;
    if id.ends_with(".sock") {
        id = &id[..id.len() - 5];
    }
    log!("Parsed sock id: {}", id);
    id.parse::<u64>().ok()
}

pub fn sub_send_msg(msg: WindowIpcMessage) {
    log!("IPC send message to main window, no response");
    let name = get_sock_name(get_sub_virt().to_owned(), None);
    log!("Try connected to main window {}", name);
    let name = name.to_ns_name::<GenericNamespaced>().unwrap();
    let mut conn = Stream::connect(name).unwrap();
    log!(
        "Send message to main window {}, {}",
        msg.window_id,
        msg.message
    );
    serde_json::to_writer(&mut conn, &msg).unwrap();
    conn.write(b"\n").unwrap();
}

pub fn sub_send_msg_wait_msg(msg: WindowIpcMessage) -> Result<WindowIpcMessage, Box<dyn Error>> {
    log!("IPC send message to main window, with wait for response");
    let name = get_sock_name(get_sub_virt().to_owned(), None);
    log!("Try connected to main window {}", name);
    let name = name.to_ns_name::<GenericNamespaced>()?;
    let mut conn = Stream::connect(name)?;
    log!(
        "Send message to main window {}, {}",
        msg.window_id,
        msg.message
    );
    serde_json::to_writer(&mut conn, &msg)?;
    conn.write(b"\n")?;
    log!("Wait for response from main window");
    let mut reader = BufReader::new(&mut conn);
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    let msg: WindowIpcMessage = serde_json::from_str(&buf)?;
    log!(
        "Received message from main window {}, {}",
        msg.window_id,
        msg.message
    );
    Ok(msg)
}

pub fn main_send_msg(msg: WindowIpcMessage, conn: &mut Stream) {
    log!("IPC send message to sub window, no response");
    log!(
        "Send message to sub window {}, {}",
        msg.window_id,
        msg.message
    );
    conn.write(serde_json::to_string(&msg).unwrap().as_bytes())
        .unwrap();
    conn.write(b"\n").unwrap();
}
