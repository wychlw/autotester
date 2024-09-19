//! This module contains the `Screen` trait and its related types.
//!
//! The `Screen` trait is used to interact with a Screen on the screen.
//! It can be defined as follows:
//! - Has a specific 2D size
//! - Can read the contents of the screen as an image(RgbaImage, GrayImage should be converted to RgbaImage)
//! - Can move the cursor to a specific position
//! - Can click the left, right, or middle mouse button
//! - Can scroll up or down (The scroll will always be biological direction)
//! - Can write text (e.g. typing in a text box)
//!
//! The `WrapperScreen` trait is used to wrap a `Screen` and provide additional functionality.
//! The `InnerScreen` trait is used to access the inner `Screen` of a `WrapperScreen`.

use std::error::Error;

use image::RgbaImage;

use crate::util::anybase::AnyBase;

/// A Screen trait, used to interact with a screen
pub trait Screen: AnyBase {
    //! Get the size of the Screen
    fn size(&self) -> (u32, u32);

    /// Read the contents of the Screen as an image
    fn read(&mut self) -> Result<RgbaImage, Box<dyn Error>>;

    /// Move the cursor to the specified position
    /// Note: (0, 0) is always the top-left corner
    fn move_to(&mut self, x: u32, y: u32) -> Result<(), Box<dyn Error>>;

    /// Click the left mouse button
    fn click_left(&mut self) -> Result<(), Box<dyn Error>>;

    /// Click the right mouse button
    fn click_right(&mut self) -> Result<(), Box<dyn Error>>;

    /// Click the middle mouse button
    fn click_middle(&mut self) -> Result<(), Box<dyn Error>>;

    /// Scroll up
    fn scroll_up(&mut self, len: u32) -> Result<(), Box<dyn Error>>;

    /// Scroll down
    fn scroll_down(&mut self, len: u32) -> Result<(), Box<dyn Error>>;

    /// Write text
    fn write(&mut self, data: String) -> Result<(), Box<dyn Error>>;

    /// Hold a key, identified by keycode
    fn hold(&mut self, key: u16) -> Result<(), Box<dyn Error>>;

    /// Release a key, identified by keycode
    fn release(&mut self, key: u16) -> Result<(), Box<dyn Error>>;

    /// Snipper for moving the cursor to a specific position and clicking the left mouse button
    fn click_left_at(&mut self, x: u32, y: u32) -> Result<(), Box<dyn Error>> {
        self.move_to(x, y)?;
        self.click_left()
    }

    /// Snipper for moving the cursor to a specific position and clicking the right mouse button
    fn click_right_at(&mut self, x: u32, y: u32) -> Result<(), Box<dyn Error>> {
        self.move_to(x, y)?;
        self.click_right()
    }
}

/// A dynamic Screen type
pub type DynScreen = Box<dyn Screen + Send>;

/// A trait for wrapping a `Screen` and providing additional functionality
pub trait WrapperScreen: Screen {
    /// Exit the Screen and return the inner Screen
    fn exit(self) -> DynScreen;
}

/// A trait for accessing the inner `Screen` of a `WrapperScreen`
pub trait InnerScreen: WrapperScreen {
    /// Get a reference to the inner Screen
    fn inner_ref(&self) -> &DynScreen;
    fn inner_mut(&mut self) -> &mut DynScreen;
}
