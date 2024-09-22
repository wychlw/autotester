#![feature(macro_metavar_expr_concat)]

use tester::{
    impl_sub_window,
    ui::main::{AppUi, SubWindow},
};

use eframe::egui::{Context, Id, Window};

pub struct TestUi {}

impl Default for TestUi {
    fn default() -> Self {
        Self {}
    }
}

impl SubWindow for TestUi {
    fn show(&mut self, ctx: &Context, title: &str, id: &Id, open: &mut bool) {
        Window::new(title)
            .id(id.to_owned())
            .open(open)
            .show(ctx, |ui| {
                ui.label("TestUi");
            });
    }
}

impl_sub_window!(TestUi, "TestUi");

fn main() {
    let _ui = AppUi::new().unwrap();
    return ();
}
