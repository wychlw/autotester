//! Capture your computer's monitor

use std::error::Error;

use enigo::{Axis, Button, Coordinate, Direction, Enigo, Keyboard, Mouse, Settings};
use image::RgbaImage;

use crate::util::anybase::AnyBase;

use super::screen::Screen;

pub struct Monitor {
    inner: xcap::Monitor,
    input: Enigo,
}

impl Monitor {
    /// Build a new `Monitor` instance.
    ///
    /// The default monitor is the primary monitor.
    /// Or you can specify the monitor by the `id`.
    pub fn build(id: Option<u32>) -> Result<Monitor, Box<dyn Error>> {
        let enigo = Enigo::new(&Settings::default())?;
        let monitors = xcap::Monitor::all()?;
        for monitor in monitors {
            if (id.is_some_and(|x| monitor.id() == x)) || monitor.is_primary() {
                return Ok(Monitor {
                    inner: monitor,
                    input: enigo,
                });
            }
        }
        Err("No primary monitor found".into())
    }
}

impl AnyBase for Monitor {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

impl Screen for Monitor {
    fn size(&self) -> (u32, u32) {
        (self.inner.width(), self.inner.height())
    }
    fn read(&mut self) -> Result<RgbaImage, Box<dyn Error>> {
        let img = self.inner.capture_image()?;
        Ok(img)
    }
    fn move_to(&mut self, x: u32, y: u32) -> Result<(), Box<dyn Error>> {
        self.input.move_mouse(x as i32, y as i32, Coordinate::Abs)?;
        Ok(())
    }
    fn click_left(&mut self) -> Result<(), Box<dyn Error>> {
        self.input.button(Button::Left, Direction::Click)?;
        Ok(())
    }
    fn click_right(&mut self) -> Result<(), Box<dyn Error>> {
        self.input.button(Button::Right, Direction::Click)?;
        Ok(())
    }
    fn click_middle(&mut self) -> Result<(), Box<dyn Error>> {
        self.input.button(Button::Middle, Direction::Click)?;
        Ok(())
    }
    fn scroll_up(&mut self, len: u32) -> Result<(), Box<dyn Error>> {
        self.input.scroll(len as i32, Axis::Vertical)?;
        Ok(())
    }
    fn scroll_down(&mut self, len: u32) -> Result<(), Box<dyn Error>> {
        self.input.scroll(-(len as i32), Axis::Vertical)?;
        todo!()
    }
    fn write(&mut self, data: String) -> Result<(), Box<dyn Error>> {
        self.input.text(&data)?;
        Ok(())
    }
    fn hold(&mut self, key: u16) -> Result<(), Box<dyn Error>> {
        self.input.raw(key, Direction::Press)?;
        Ok(())
    }
    fn release(&mut self, key: u16) -> Result<(), Box<dyn Error>> {
        self.input.raw(key, Direction::Release)?;
        Ok(())
    }
}
