#![feature(box_into_inner)]
#![feature(macro_metavar_expr_concat)]
#![allow(clippy::module_inception)]

pub mod consts;
pub mod cli;
pub mod gui;
pub mod exec;
pub mod devhost {
    pub mod devhost;
    pub mod sdwirec;
}
pub mod device {
    pub mod device;
}
pub mod util {
    pub mod anybase;
    pub mod logger;
    pub mod util;
}
pub mod pythonapi;
pub mod vendor;
pub mod ui;
