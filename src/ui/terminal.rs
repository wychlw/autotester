use std::{
    cmp::{max, min},
    error::Error,
    io::{BufRead, BufReader, ErrorKind},
    ops::Range,
    sync::{Arc, Mutex},
    thread::{sleep, spawn, JoinHandle},
    time::Duration,
};

use eframe::egui::{
    scroll_area::ScrollBarVisibility, Context, FontId, Id, RichText, ScrollArea, Ui, Window,
};
use interprocess::local_socket::{
    traits::ListenerExt, GenericNamespaced, Listener, ListenerNonblockingMode, ListenerOptions,
    Stream, ToNsName,
};

use crate::{consts::DURATION, err, impl_sub_window, info, log, ui::ipc::main_send_msg};

use super::{ipc::WindowIpcMessage, main::SubWindow, ui_cli_exec::UiCliIpcMsg};

pub struct Terminal {
    size: (u32, u32),
    buf: Arc<Mutex<Vec<u8>>>,
    listener: Arc<Mutex<Option<Listener>>>,
    handle: Option<JoinHandle<()>>,
    stop: Arc<Mutex<bool>>,
}

impl Default for Terminal {
    fn default() -> Self {
        let test_data = b"Hello, World!\n".to_vec();

        let buf = Arc::new(Mutex::new(test_data));
        let mut res = Terminal {
            size: (24, 80),
            buf,
            listener: Arc::new(Mutex::new(None)),
            handle: None,
            stop: Arc::new(Mutex::new(false)),
        };

        let buf = res.buf.clone();
        let listener = res.listener.clone();
        let stop = res.stop.clone();

        let handler = spawn(move || loop {
            sleep(Duration::from_millis(DURATION));
            {
                let stop = stop.lock().unwrap();
                if *stop {
                    break;
                }
            }
            {
                let mut _listener = listener.lock().unwrap();
                if _listener.is_none() {
                    continue;
                }
                let listener = _listener.as_mut().unwrap();
                let mut end = false;
                for m in listener.incoming() {
                    match m {
                        Err(e) if e.kind() == ErrorKind::WouldBlock => {
                            break;
                        }
                        Err(e) => {
                            err!("IPC error: {}", e);
                            continue;
                        }
                        Ok(m) => {
                            let mut reader = BufReader::new(m);
                            let mut b = String::new();
                            match reader.read_line(&mut b) {
                                // Err(e) if e.kind() == ErrorKind::WouldBlock => {
                                //     break;
                                // }
                                Err(e) => {
                                    err!("IPC Read error: {}", e);
                                    continue;
                                }
                                Ok(_) => {}
                            };
                            info!("Terminal got message: {:?}", b);
                            let msg: UiCliIpcMsg = match serde_json::from_str(&b) {
                                Err(e) => {
                                    err!("IPC message decode error: {}", e);
                                    continue;
                                }
                                Ok(m) => m,
                            };
                            match msg {
                                UiCliIpcMsg::BUILD(_) => {
                                    err!("Already hooked, unexpected message");
                                    unreachable!();
                                }
                                UiCliIpcMsg::REBUILD(_) => {
                                    err!("Unexpected message, you should not send REBUILD if not broken");
                                    unreachable!();
                                }
                                UiCliIpcMsg::CONSOLE(data) => {
                                    let mut buf = buf.lock().unwrap();
                                    buf.extend(data);
                                }
                                UiCliIpcMsg::EXIT => {
                                    end = true;
                                    break;
                                }
                            }
                        }
                    }
                }
                if end {
                    _listener.take();
                }
            }
        });

        res.handle = Some(handler);

        res
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let mut stop = self.stop.lock().unwrap();
        *stop = true;
    }
}

impl Terminal {
    pub fn try_hook(&mut self, msg: WindowIpcMessage) -> Result<(), Box<dyn Error>> {
        let listener = self.listener.clone();
        let mut listener = listener.lock().unwrap();
        if listener.is_some() {
            return Err("Terminal already hooked".into());
        }
        let msg = msg.message;
        let msg = serde_json::from_str::<UiCliIpcMsg>(&msg)?;
        let sock_name = match msg {
            UiCliIpcMsg::BUILD(sock_name) => sock_name,
            _ => return Err("Invalid message".into()),
        };
        let sock_name = sock_name.to_ns_name::<GenericNamespaced>()?;
        let opts = ListenerOptions::new()
            .name(sock_name)
            .nonblocking(ListenerNonblockingMode::Both);
        let lst = opts.create_sync()?;
        *listener = Some(lst);
        Ok(())
    }
    pub fn un_hook(&mut self) -> Result<(), Box<dyn Error>> {
        let listener = self.listener.clone();
        let mut listener = listener.lock().unwrap();
        if listener.is_none() {
            return Ok(());
        }
        let _listener = listener.take().unwrap();
        Ok(())
    }
}

impl Terminal {
    fn gc_bufs(&mut self) {
        // Remove some old data
        let buf = self.buf.clone();
        let mut buf = buf.lock().unwrap();
        let lines = buf.iter().filter(|&&c| c == b'\n').count();
        if lines < 100 {
            return;
        }
        let drain_line = lines - 100;
        let lst_pos = buf
            .iter()
            .enumerate()
            .filter(|&(_, &c)| c == b'\n')
            .nth(drain_line)
            .unwrap()
            .0;
        buf.drain(0..lst_pos);
    }
    fn render_row(&mut self, ui: &mut Ui, rows: Range<usize>) {
        self.gc_bufs();
        ui.label("                                                                                                                                                                ");
        let buf = self.buf.clone();
        let buf = buf.lock().unwrap();
        let mut data = Vec::from([0]);
        data.extend(
            buf.iter()
                .enumerate()
                .filter(|&(_, &c)| c == b'\n')
                .map(|(i, _)| i),
        );
        if !buf.is_empty() && !(buf[buf.len() - 1] == b'\n') {
            data.push(buf.len());
        }
        let begin_line = min(data.len() - 1, rows.start);
        let end_line = min(data.len() - 1, rows.end);
        let begin_pos = data[begin_line];
        let end_pos = data[end_line];
        let row = &buf[begin_pos..end_pos];
        let row = String::from_utf8_lossy(row);
        let text = RichText::new(row).monospace().font(FontId::monospace(14.0));
        ui.label(text);
    }

    fn render_term(&mut self, ui: &mut Ui) {
        // For now, the term has no style... Mabe Ansi to egui style?

        let total_rows = {
            let buf = self.buf.clone();
            let buf = buf.lock().unwrap();
            buf.iter().filter(|&&c| c == b'\n').count()
        };
        let total_rows = max(self.size.0 as usize, total_rows);

        ScrollArea::vertical()
            .max_height(self.size.0 as f32 * 18.0)
            .max_width(self.size.1 as f32 * 18.0)
            .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
            .stick_to_bottom(true)
            .show_rows(ui, 14.0, total_rows, |ui, rows| self.render_row(ui, rows));

        ui.ctx().request_repaint();
    }
    fn show(&mut self, ui: &mut Ui) {
        ui.label("Terminal");

        self.render_term(ui);
    }
}

impl SubWindow for Terminal {
    fn show(&mut self, ctx: &Context, title: &str, id: &Id, open: &mut bool) {
        let window = Window::new(title)
            .id(id.to_owned())
            .open(open)
            .hscroll(false)
            .vscroll(false)
            .min_height(self.size.0 as f32 * 20.0)
            .min_width(self.size.1 as f32 * 20.0)
            .resizable([false, false]);
        window.show(ctx, |ui| {
            self.show(ui);
        });
    }
    fn on_ipc(&mut self, msg: &str, _conn: &mut Stream) {
        log!("Terminal got message: {}", msg);
        let msg = serde_json::from_str::<UiCliIpcMsg>(msg);
        match msg {
            Ok(msg) => match msg {
                UiCliIpcMsg::BUILD(s_name) => {
                    let sock_name = s_name.clone().to_ns_name::<GenericNamespaced>().unwrap();
                    let opts = ListenerOptions::new()
                        .name(sock_name)
                        .nonblocking(ListenerNonblockingMode::Both);
                    let listener = opts.create_sync().unwrap();
                    let self_listener = self.listener.clone();
                    let mut self_listener = self_listener.lock().unwrap();
                    if self_listener.is_some() {
                        err!("Already hooked, unexpected message");
                        main_send_msg(
                            WindowIpcMessage {
                                window_id: 0,
                                message: serde_json::to_string(&UiCliIpcMsg::EXIT).unwrap(),
                            },
                            _conn,
                        );
                        return;
                    }
                    *self_listener = Some(listener);
                    info!("Terminal hooked with sock {}", s_name);
                    {
                        let mut buf = self.buf.lock().unwrap();
                        buf.clear();
                    }
                    main_send_msg(
                        WindowIpcMessage {
                            window_id: 0,
                            message: serde_json::to_string(&UiCliIpcMsg::BUILD(s_name)).unwrap(),
                        },
                        _conn,
                    );
                }
                UiCliIpcMsg::REBUILD(s_name) => {
                    let sock_name = s_name.clone().to_ns_name::<GenericNamespaced>().unwrap();
                    let opts = ListenerOptions::new()
                        .name(sock_name)
                        .nonblocking(ListenerNonblockingMode::Both);
                    let listener = opts.create_sync().unwrap();
                    let self_listener = self.listener.clone();
                    let mut self_listener = self_listener.lock().unwrap();
                    *self_listener = Some(listener);
                    log!("Terminal rebuild with sock {}", s_name);
                    main_send_msg(
                        WindowIpcMessage {
                            window_id: 0,
                            message: serde_json::to_string(&UiCliIpcMsg::REBUILD(s_name)).unwrap(),
                        },
                        _conn,
                    );
                }
                UiCliIpcMsg::CONSOLE(msg) => {
                    let mut buf = self.buf.lock().unwrap();
                    buf.extend(msg);
                    // The other way has some issue now... Use the main ipc as a workaround
                    // err!("Unexpected message, you should not send CONSOLE message at handshake");
                    // unreachable!();
                }
                UiCliIpcMsg::EXIT => {
                    err!("Unexpected message, you should not send EXIT message at handshake");
                    unreachable!();
                }
            },
            Err(e) => {
                err!("Terminal IPC message decode error: {}", e);
            }
        }
    }
}

impl_sub_window!(Terminal, "Terminal");
