//! Object to match the GUI
//!
//! We can't do the same thing as the CLI part: give a string and wait for
//! it. We need to match the GUI with a needle.

use image::RgbaImage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Needle {
    /// Basic needle, just compare the screen with given image to see if the similarity is enough
    Basic(Vec<Area>),
    // In future, consider adding more needle types,
    // like using neural network to match the screen
    // to get a better flexibility
    NeedleEnd, // remove will warn about unreachable pattern
}

impl Needle {
    pub fn is_basic(&self) -> bool {
        matches!(self, Needle::Basic(_))
    }
}

/// Needle type
/// OpenQA needle compatible, with same definition
///
/// See <https://open.qa/docs/#_needle>
#[derive(Serialize, Deserialize)]
pub enum NeedleType {
    #[serde(rename = "match")]
    Match,
    #[serde(rename = "ocr")]
    Ocr, // Currently not supported
    #[serde(rename = "exclude")]
    Exclude,
}

/// Click point
/// OpenQA needle compatible, with same definition
///
/// See <https://open.qa/docs/#_needle>
#[derive(Serialize, Deserialize)]
pub struct ClickPoint {
    pub xpos: u32,
    pub ypos: u32,
}

/// Area to match
/// OpenQA needle compatible, with same definition
///
/// See <https://open.qa/docs/#_needle>
#[derive(Serialize, Deserialize)]
pub struct Area {
    /// X coordinate of the top left corner
    pub x: u32,
    /// Y coordinate of the top left corner
    pub y: u32,
    /// Width of the area
    pub width: u32,
    /// Height of the area
    pub height: u32,
    /// Type of the needle
    pub needle: NeedleType,
    /// The similarity threshold
    #[serde(rename = "match")]
    pub match_threhold: f32,
    /// The click point
    pub click_point: Option<ClickPoint>,
    /// The image to match
    ///
    /// This field should be auto added by needle readers
    #[serde(skip_serializing, skip_deserializing)]
    pub target: Option<RgbaImage>,
}
