//! Handler API for GUI

use image::RgbaImage;

use crate::exec::needle::Needle;

pub trait GuiHandler {
    /// Check if this handler can handle this type of needle
    fn can_handle(&self, needle: &Needle) -> bool;
    /// Handle the needle, return if the screen matches the needle
    fn handle(&self, needle: &Needle, screen: &RgbaImage) -> bool;
    // Check if this handler can handle screen change
    fn can_handle_change(&self, allow_list: Option<&[String]>) -> bool;
    /// Handle the screen change, return if the screen matches the previous screen
    fn handle_change(&self, screen: &RgbaImage, prev_screen: &RgbaImage) -> bool;
}

pub type DynGuiHandler = dyn GuiHandler + Send + Sync + 'static;

pub struct HandlerCollector {
    pub inner: &'static DynGuiHandler,
}

inventory::collect!(HandlerCollector);
