use std::{fs::File, io::Write};

use eframe::egui::{Context, Id, ScrollArea, TextEdit, TextStyle, Ui, Window};
use egui_extras::syntax_highlighting::{highlight, CodeTheme};

use crate::{err, impl_sub_window};

use super::{main::SubWindow, pyenv::PyEnv};

pub struct CodeEditor {
    code: String,
    save_to: String,
    pyenv: PyEnv,
}

impl Default for CodeEditor {
    fn default() -> Self {
        Self {
            code: "\
from tester import *

print('Hello, world!')
".to_string(),
            save_to: "".to_string(),
            pyenv: PyEnv::default(),
        }
    }
}

impl CodeEditor {
    fn editor(&mut self, ui: &mut Ui) {
        let mut layout = |ui: &Ui, string: &str, width: f32| {
            let theme = CodeTheme::from_style(ui.style());
            let mut job = highlight(ui.ctx(), &theme, string, "Python");
            job.wrap.max_width = width;
            ui.fonts(|f| f.layout_job(job))
        };

        ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                TextEdit::multiline(&mut self.code)
                    .font(TextStyle::Monospace)
                    .code_editor()
                    .desired_rows(25)
                    .lock_focus(true)
                    .desired_width(f32::INFINITY)
                    .layouter(&mut layout),
            )
        });
    }
    fn run_code(&mut self) {
        self.pyenv.run_code(&self.code);
    }
    fn save_code(&mut self, ui: &mut Ui) {
        if ui.button("Write to file").clicked() {
            println!("Write to file: {}", self.save_to);
            let f = File::options()
                .append(true)
                .create(true)
                .open(&self.save_to);
            match f {
                Ok(mut file) => {
                    let e = file.write_all(self.code.as_bytes());
                    if let Err(e) = e {
                        err!("Write to file error: {}", e);
                    } else {
                        self.code.clear();
                    }
                }
                Err(e) => {
                    err!("Open file error: {}", e);
                }
            }
        }
    }
    fn bottom_butt(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("Run").clicked() {
                self.run_code();
            }
            self.save_code(ui);
        });
    }
    fn bottom(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Save to:");
                ui.text_edit_singleline(&mut self.save_to);
            });
            self.bottom_butt(ui);
        });
    }
    fn show(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            self.editor(ui);
            self.bottom(ui);
        });
    }
}

impl SubWindow for CodeEditor {
    fn show(&mut self, ctx: &Context, title: &str, id: &Id, open: &mut bool) {
        let window = Window::new(title).id(id.to_owned()).open(open).resizable([true, true]);
        window.show(ctx, |ui| {
            self.show(ui);
        });
    }
}

impl_sub_window!(CodeEditor, "CodeEditor");
