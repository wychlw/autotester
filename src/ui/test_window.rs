use eframe::egui::{Context, Id, Window};
use interprocess::local_socket::{prelude::*, GenericNamespaced, Stream, ToNsName};

use crate::{
    impl_sub_window, info,
    ui::{
        ipc::{get_sock_name, WindowIpcMessage},
        util::get_main_virt,
    },
};

use super::{ipc::sub_send_msg, main::SubWindow, util::__init_sub_virt__};

pub struct TestUi {}

impl Default for TestUi {
    fn default() -> Self {
        Self {}
    }
}

fn create_test_ipc() {
    let name = get_sock_name(get_main_virt().to_owned(), None);
    let name = name.to_ns_name::<GenericNamespaced>().unwrap();
    let conn = Stream::connect(name).unwrap();
    let msg = WindowIpcMessage {
        window_id: 0,
        message: "Hello, World!".to_owned(),
    };
    serde_json::to_writer(conn, &msg).unwrap();
    info!("TestUi sent message: {}", msg.message);
}

impl SubWindow for TestUi {
    fn show(&mut self, ctx: &Context, title: &str, id: &Id, open: &mut bool) {
        __init_sub_virt__(&get_main_virt());
        Window::new(title)
            .id(id.to_owned())
            .open(open)
            .show(ctx, |ui| {
                ui.label("TestUi");
                if ui.button("Send IPC").clicked() {
                    create_test_ipc();
                    sub_send_msg(WindowIpcMessage {
                        window_id: id.value(),
                        message: "Hello, World Self!".to_owned(),
                    });
                }
            });
    }
    fn on_ipc(&mut self, msg: &str, _conn: &mut Stream) {
        info!("TestUi got message: {}", msg);
    }
}

impl_sub_window!(TestUi, "TestUi");
