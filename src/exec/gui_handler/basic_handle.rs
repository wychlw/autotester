//! Basic handler for GUI. Which also means OpenQA compatible.


use image::RgbaImage;

use crate::exec::{
    gui_handler::handler_api::HandlerCollector,
    needle::{Area, Needle, NeedleType},
};

use super::handler_api::GuiHandler;

pub struct BasicHandler {}

pub fn basic_handle_once(area: &Area, screen: &RgbaImage) -> bool {
    let target = match &area.target {
        Some(target) => target,
        None => return false,
    };
    if target.width() != screen.width() || target.height() != screen.height() {
        return false;
    }
    match area.needle {
        NeedleType::Match => {
            let mut match_count = 0;
            let total_count = area.width * area.height;
            for x in 0..area.width {
                for y in 0..area.height {
                    let x = area.x + x;
                    let y = area.y + y;
                    let pixel = screen.get_pixel(x, y);
                    let target_pixel = target.get_pixel(x, y);
                    if pixel == target_pixel {
                        match_count += 1;
                    }
                }
            }
            let similarity = match_count as f32 / total_count as f32;
            similarity >= area.match_threhold
        }
        NeedleType::Ocr => return false,
        NeedleType::Exclude => {
            let mut match_count = 0;
            let total_count = screen.width() * screen.height() - area.width * area.height;
            for x in 0..screen.width() {
                for y in 0..screen.height() {
                    if x >= area.x
                        && x < area.x + area.width
                        && y >= area.y
                        && y < area.y + area.height
                    {
                        continue;
                    }
                    let pixel = screen.get_pixel(x, y);
                    let target_pixel = target.get_pixel(x, y);
                    if pixel == target_pixel {
                        match_count += 1;
                    }
                }
            }
            let similarity = match_count as f32 / total_count as f32;
            similarity >= area.match_threhold
        }
    }
}

impl GuiHandler for BasicHandler {
    fn can_handle(&self, needle: &Needle) -> bool {
        needle.is_basic()
    }
    fn handle(&self, needle: &Needle, screen: &RgbaImage) -> bool {
        let needle = match needle {
            Needle::Basic(areas) => areas,
            _ => return false,
        };
        needle.iter().all(|area| basic_handle_once(area, screen))
    }
    fn can_handle_change(&self, allow_list: Option<&[String]>) -> bool {
        if let Some(allow_list) = allow_list {
            allow_list.iter().any(|x| x == "basic")
        } else {
            true
        }
    }
    fn handle_change(&self, screen: &RgbaImage, prev_screen: &RgbaImage) -> bool {
        for x in 0..screen.width() {
            for y in 0..screen.height() {
                let pixel = screen.get_pixel(x, y);
                let prev_pixel = prev_screen.get_pixel(x, y);
                if pixel != prev_pixel {
                    return false;
                }
            }
        }
        true
        // screen != prev_screen
    }
}

inventory::submit! {
    HandlerCollector {
        inner: &BasicHandler {}
    }
}
