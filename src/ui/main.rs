//! Main UI render for the APP

use std::{error::Error, thread::sleep, time::Duration};

use eframe::{
    egui::{Context, Id, SidePanel, Ui, ViewportBuilder},
    run_native, App, Frame, NativeOptions,
};
use interprocess::local_socket::{Listener, Stream};

use crate::{info, util::anybase::AnyBase};

use super::ipc::init_ipc;

/// Main UI struct
///
/// NOTICE! NOTICE! This will block the main thread. If you have any other tasks to do, please run them in a separate thread.
/// Or, use IPC to communicate with the UI process.
#[derive(Default)]
pub struct AppUi {}

impl AppUi {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let options = NativeOptions {
            viewport: ViewportBuilder::default()
                .with_title("AutoTestor")
                .with_inner_size([800.0, 600.0]),
            ..Default::default()
        };

        run_native(
            "AutoTestor",
            options,
            Box::new(|_cc| Ok(Box::<MyApp>::default())),
        )?;

        Ok(AppUi {})
    }
}

pub struct SubWindowHolder {
    pub window: Box<dyn SubWindow>,
    pub id: Id,
    pub idx: u64,
    pub title: String,
    pub open: bool,
}

pub struct MyApp {
    sub_window_creator: Vec<Box<DynSubWindowCreator>>, // We ensure that the sub windows only work in the main thread
    pub(super) sub_windows: Vec<SubWindowHolder>,
    sub_window_idx: usize,
    pub(super) listener: Listener,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut sub_window_creator = Vec::new();
        for c in inventory::iter::<SubWindowCollector> {
            let f = c.create;
            let w = f();
            sub_window_creator.push(w);
        }
        Self {
            sub_window_creator,
            sub_windows: Vec::new(),
            sub_window_idx: 0,
            listener: init_ipc().unwrap(),
        }
    }
}

impl MyApp {
    fn sub_window_pannel(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.label("SubWindow Panel");
        ui.vertical(|ui| {
            for creator in &self.sub_window_creator {
                let name = creator.name();
                if ui.button(name).clicked() {
                    let idx = self.sub_window_idx as u64;
                    self.sub_window_idx += 1;
                    let title = format!("{}: {}", name, idx);
                    let id = Id::new(idx);
                    info!("Try create sub window: {}", title);
                    sleep(Duration::from_millis(5));
                    let sub_windows = &mut self.sub_windows;
                    sub_windows.push(SubWindowHolder {
                        window: creator.open(),
                        id,
                        idx,
                        title,
                        open: true,
                    });
                }
            }
            {
                sleep(Duration::from_millis(5));
                let sub_windows = &mut self.sub_windows;
                sub_windows.retain(|w| w.open);
                for w in sub_windows.iter_mut() {
                    w.window.show(ctx, &w.title, &w.id, &mut w.open);
                }
            }
        });
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        let _ = frame;
        self.handle_ipc();
        SidePanel::right("SubWindow Panel")
            .default_width(200.0)
            .show(ctx, |ui| {
                self.sub_window_pannel(ctx, ui);
            });
    }
}

/// SubWindow trait
///
/// This trait is used to hold the sub window.
/// This is because the egui is a immediate mode UI. when you display a new window, it will be shown THAT particular frame. if you need that window to stay, you need to create a new window AGAIN next frame too and egui using the window's name (or other id source), egui will internally keep track of its position / focus status etc..
/// So, we need some way to keep track of the sub window.
pub trait SubWindow: AnyBase {
    /// Show the window, this will be called every frame. Your window is identified by the `id` parameter.
    /// However, that doesn't mean you should change the title, as this contains the window number, useful for the user.
    fn show(&mut self, ctx: &Context, title: &str, id: &Id, open: &mut bool);

    /// For IPC, this will be called when the IPC message is received.
    /// The message is a string, you can use it as you like.
    fn on_ipc(&mut self, msg: &str, stream: &mut Stream);
}

#[doc(hidden)]
pub trait SubWindowCreator {
    fn name(&self) -> &str;
    fn open(&self) -> Box<dyn SubWindow>;
}

/// Snippet to register a sub window
///
/// # Arguments
/// $name: The struct name of the sub window
/// $window_name: The name of the window, will become the title of the window
///
/// # Example
/// `impl_sub_window!(TestUiStruct, "TestUiName");`
/// where TestUiStruct implements SubWindow trait and Default trait
///
/// # Notice
/// If you found rust-analyzer gives "invalid metavariable expression", this is a nightly feature, you can ignore it. It will work.
/// The problem is on `${concat()}` macro. Just suppress it.
#[macro_export]
macro_rules! impl_sub_window {
    ($name:ident, $window_name:literal) => {
        struct ${concat($name, Creator)} {}

        impl $crate::ui::main::SubWindowCreator for ${concat($name, Creator)} {
            fn name(&self) -> &str {
                $window_name
            }
            fn open(&self) -> Box<dyn $crate::ui::main::SubWindow> {
                Box::new($name::default())
            }
        }

        #[allow(non_snake_case)]
        fn ${concat(create_, $name)}() -> Box<$crate::ui::main::DynSubWindowCreator> {
            Box::new(${concat($name, Creator)} {})
        }

        inventory::submit! {
            $crate::ui::main::SubWindowCollector {
                create: ${concat(create_, $name)},
            }
        }

        $crate::impl_any!($name);
    };
}

/// Type should return from the creator.
#[doc(hidden)]
pub type DynSubWindowCreator = dyn SubWindowCreator + Send + Sync + 'static;

#[doc(hidden)]
/// We need to use a function to create the sub window creator on start time.
pub type SubWindowCreatorCreator = fn() -> Box<DynSubWindowCreator>;

#[doc(hidden)]
pub struct SubWindowCollector {
    pub create: SubWindowCreatorCreator,
}

inventory::collect!(SubWindowCollector);
