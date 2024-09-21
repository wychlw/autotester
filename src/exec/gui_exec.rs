//! Executor gor GUI
//!

use std::{any::Any, error::Error, time::Instant};

use image::RgbaImage;

use crate::{
    exec::gui_handler::{
        basic_handle::basic_handle_once,
        handler_api::HandlerCollector,
    },
    gui::screen::{DynScreen, Screen},
    info,
    util::anybase::AnyBase,
};

use super::{gui_api::GuiTestApi, needle::Needle};

pub struct GuiTestor {
    inner: DynScreen,
}

impl GuiTestor {
    pub fn build(inner: DynScreen) -> GuiTestor {
        GuiTestor { inner }
    }
}

impl AnyBase for GuiTestor {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Screen for GuiTestor {
    fn size(&self) -> (u32, u32) {
        self.inner.size()
    }
    fn read(&mut self) -> Result<RgbaImage, Box<dyn Error>> {
        self.inner.read()
    }
    fn move_to(&mut self, x: u32, y: u32) -> Result<(), Box<dyn Error>> {
        self.inner.move_to(x, y)
    }
    fn click_left(&mut self) -> Result<(), Box<dyn Error>> {
        self.inner.click_left()
    }
    fn click_right(&mut self) -> Result<(), Box<dyn Error>> {
        self.inner.click_right()
    }
    fn click_middle(&mut self) -> Result<(), Box<dyn Error>> {
        self.inner.click_middle()
    }
    fn scroll_up(&mut self, len: u32) -> Result<(), Box<dyn Error>> {
        self.inner.scroll_up(len)
    }
    fn scroll_down(&mut self, len: u32) -> Result<(), Box<dyn Error>> {
        self.inner.scroll_down(len)
    }
    fn write(&mut self, data: String) -> Result<(), Box<dyn Error>> {
        self.inner.write(data)
    }
    fn hold(&mut self, key: u16) -> Result<(), Box<dyn Error>> {
        self.inner.hold(key)
    }
    fn release(&mut self, key: u16) -> Result<(), Box<dyn Error>> {
        self.inner.release(key)
    }
}

struct TimeOutErr;

impl Error for TimeOutErr {}

impl std::fmt::Debug for TimeOutErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Timeout")
    }
}

impl std::fmt::Display for TimeOutErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Timeout")
    }
}

/// Wait for a timeout and do something
///
/// func should return `Some(T)` if the operation is done, `None` if the operation is not done yet, and `Err(E)` if an error occurred.
fn wait_timeout_do<T>(
    timeout: u32,
    func: &mut dyn FnMut() -> Result<Option<T>, Box<dyn Error>>,
) -> Result<T, Box<dyn Error>> {
    let begin = Instant::now();
    loop {
        match func() {
            Ok(Some(v)) => return Ok(v),
            Ok(None) => {}
            Err(e) => return Err(e),
        }
        if begin.elapsed().as_secs() >= timeout as u64 {
            return Err(TimeOutErr.into());
        }
    }
}

impl GuiTestApi for GuiTestor {
    fn assert_screen(&mut self, needle: &Needle, timeout: u32) -> Result<(), Box<dyn Error>> {
        info!("Waiting for screen...");
        wait_timeout_do(timeout, &mut || {
            let screen = self.read()?;
            let mut res = None;
            for handler in inventory::iter::<HandlerCollector> {
                if !handler.inner.can_handle(&needle) {
                    continue;
                }
                res = Some(handler.inner.handle(&needle, &screen) || res.unwrap_or(false));
            }
            match res {
                Some(true) => Ok(Some(())),
                Some(false) => Ok(None),
                None => Err("No handler found".into()),
            }
        })
    }
    fn assert_screen_click(&mut self, needle: &Needle, timeout: u32) -> Result<(), Box<dyn Error>> {
        self.assert_screen(&needle, timeout)?;
        if !needle.is_basic() {
            return Err("Only Basic Needle support this".into());
        }
        let areas = match needle {
            Needle::Basic(areas) => areas,
            _ => unreachable!(),
        };
        let screen = self.read()?;
        for area in areas {
            if area.click_point.is_none() {
                continue;
            }
            if !basic_handle_once(area, &screen) {
                continue;
            }
            let point = area.click_point.as_ref().unwrap();
            self.click_left_at(point.xpos, point.ypos)?;
        }
        Ok(())
    }
    fn wait_screen_change(
        &mut self,
        timeout: u32,
        allow_list: Option<&[String]>,
    ) -> Result<(), Box<dyn Error>> {
        info!("Waiting for screen change...");
        let mut prev_screen = self.read()?;
        wait_timeout_do(timeout, &mut || {
            let screen = self.read()?;
            let mut res = None;
            for handler in inventory::iter::<HandlerCollector> {
                if !handler.inner.can_handle_change(allow_list) {
                    continue;
                }
                res = Some(
                    handler.inner.handle_change(&screen, &prev_screen) || res.unwrap_or(false),
                );
            }
            match res {
                Some(true) => Ok(Some(())),
                Some(false) => {
                    prev_screen = screen;
                    Ok(None)
                }
                None => Err("No handler found".into()),
            }
        })
    }
    fn wait_still_screen(
        &mut self,
        timeout: u32,
        allow_list: Option<&[String]>,
    ) -> Result<(), Box<dyn Error>> {
        info!("Waiting for still screen...");
        let mut prev_screen = self.read()?;
        let changed = wait_timeout_do(timeout, &mut || {
            let screen = self.read()?;
            let mut res = None;
            for handler in inventory::iter::<HandlerCollector> {
                if !handler.inner.can_handle_change(allow_list) {
                    continue;
                }
                res = Some(
                    handler.inner.handle_change(&screen, &prev_screen) || res.unwrap_or(false),
                );
            }
            match res {
                Some(true) => Ok(Some(())),
                Some(false) => {
                    prev_screen = screen;
                    Ok(None)
                }
                None => Err("No handler found".into()),
            }
        });
        match changed {
            Ok(_) => Err("Screen changed".into()),
            Err(e) if e.to_string() == "Timeout" => Ok(()),
            Err(e) => Err(e),
        }
    }
}
