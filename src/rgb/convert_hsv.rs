#!/usr/bin/env rust


use crate::hsl::HSL;
use crate::hsv::HSV;
use crate::rgb::RGB;


/// Convert from `HSV` to `RGB`
///
/// **Note** uses `HSL::from(hsv)` internally
impl From<HSV> for RGB {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::rgb::RGB;
    /// use color_operators::hsv::HSV;
    ///
    /// let hsv = HSV::new(0.0, 1.0, 1.0);
    /// let rgb = RGB::from(hsv);
    ///
    /// assert_eq!(rgb.get("red"), Ok(255));
    /// assert_eq!(rgb.get("green"), Ok(0));
    /// assert_eq!(rgb.get("blue"), Ok(0));
    /// ```
    fn from(hsv: HSV) -> Self {
        Self::from(HSL::from(hsv))
    }
}

