//! Api for GUI part testing
//! 
//! For GUI part, basicially we need to check the following:
//! - Is the current screen is the expected screen?
//! - Does the screen have the expected elements?
//! - Can we interact with the screen?

use std::error::Error;

use crate::gui::screen::Screen;

use super::needle::Needle;

/// The API can used for testing, with bypass [`Screen`] operations.
pub trait GuiTestApi: Screen {
    /// Check if the current screen is the expected screen
    fn assert_screen(&mut self, needle: Needle) -> Result<(), Box<dyn Error>>;
    /// Check and click the target position
    /// 
    /// Suggest using assert_screen and click seperately, as futher consider adding relative position etc.
    fn assert_screen_click(&mut self, needle: Needle) -> Result<(), Box<dyn Error>>;

    /// Wait until current screen changed
    fn wait_screen_change(&mut self, timeout: u32) -> Result<(), Box<dyn Error>>;

    /// Wait and assert the screen won't change in timeout
    fn wait_still_screen(&mut self, timeout: u32) -> Result<(), Box<dyn Error>>;
}
