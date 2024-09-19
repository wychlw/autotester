//! This module contains all the GUI related code.
//!
//! The concept of GUI can be seen as an interactive [`screen`]:
//! - Has a specific 2D size
//! - Can read the contents of the screen as an image(RgbImage, GrayImage should be converted to RgbImage)
//! - Can move the cursor to a specific position
//! - Can click the left, right, or middle mouse button
//! - Can scroll up or down (The scroll will always be biological direction)
//! - Can write text (e.g. typing in a text box)

pub mod screen;
pub mod monitor;