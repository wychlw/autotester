use eframe::egui::{Context, Id, Ui, Window};

use crate::impl_sub_window;

use super::main::SubWindow;

pub struct Terminal {
    size: (u32, u32),
}

impl Default for Terminal {
    fn default() -> Self {
        Terminal { size: (24, 80) }
    }
}

impl Terminal {
    fn show(&mut self, ui: &mut Ui) {
        ui.label("Terminal");
    }
}

impl SubWindow for Terminal {
    fn show(&mut self, ctx: &Context, title: &str, id: &Id, open: &mut bool) {
        let window = Window::new(title)
            .id(id.to_owned())
            .open(open)
            .resizable([false, false]);
        window.show(ctx, |ui| {
            self.show(ui);
        });
    }
}

impl_sub_window!(Terminal, "Terminal");
