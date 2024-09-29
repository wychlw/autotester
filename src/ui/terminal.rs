use std::{
    cmp::{max, min},
    error::Error,
    ops::Range,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{sleep, spawn, JoinHandle},
    time::Duration,
};

use eframe::egui::{
    scroll_area::ScrollBarVisibility, Context, FontId, Id, RichText, ScrollArea, Ui, Window,
};

use crate::{consts::DURATION, impl_sub_window};

use super::main::SubWindow;

pub enum TerminalMessage {
    Data(Vec<u8>),
    Close,
}

pub struct Terminal {
    size: (u32, u32),
    buf: Arc<Mutex<Vec<u8>>>,
    send: Arc<Mutex<Option<Sender<TerminalMessage>>>>, // Only Close will be sent, indicating that the terminal is closed
    recv: Arc<Mutex<Option<Receiver<TerminalMessage>>>>,
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
            send: Arc::new(Mutex::new(None)),
            recv: Arc::new(Mutex::new(None)),
            handle: None,
            stop: Arc::new(Mutex::new(false)),
        };

        let buf = res.buf.clone();
        let send = res.send.clone();
        let recv = res.recv.clone();
        let stop = res.stop.clone();

        let handler = spawn(move || loop {
            sleep(Duration::from_millis(DURATION));
            {
                let stop = stop.lock().unwrap();
                if *stop {
                    break;
                }
            }
            let mut recv_o = recv.lock().unwrap();
            if recv_o.is_none() {
                continue;
            }
            let recv = recv_o.as_mut().unwrap();
            match recv.try_recv() {
                Ok(TerminalMessage::Data(data)) => {
                    let mut buf = buf.lock().unwrap();
                    buf.extend(data);
                }
                Ok(TerminalMessage::Close) => {
                    let mut send_o = send.lock().unwrap();
                    let send = send_o.as_ref().unwrap();
                    send.send(TerminalMessage::Close).unwrap();
                    recv_o.take();
                    send_o.take();
                    let mut buf = buf.lock().unwrap();
                    buf.clear();
                    buf.extend(b"Hello, world!\n");
                }
                Err(_) => {
                    continue;
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
        let send = self.send.lock().unwrap();
        if let Some(send) = send.as_ref() {
            send.send(TerminalMessage::Close).unwrap();
        }
    }
}

impl Terminal {
    pub fn try_hook(
        &mut self,
        recv: Receiver<TerminalMessage>,
    ) -> Result<Receiver<TerminalMessage>, Box<dyn Error>> {
        let mut recv_o = self.recv.lock().unwrap();
        if recv_o.is_some() {
            return Err("Terminal already hooked".into());
        }
        *recv_o = Some(recv);
        let (rsend, rrecv) = mpsc::channel();
        let mut send = self.send.lock().unwrap();
        *send = Some(rsend);
        Ok(rrecv)
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
        // ui.label("01234567890123456789012345678901234567890123456789012345678901234567890123456789");
        // Use this to make the line above the same length as the terminal
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
        if !buf[buf.len() - 1] == b'\n' {
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
}

impl_sub_window!(Terminal, "Terminal");
