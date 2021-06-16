#!/usr/bin/env rust


use crate::rgb::RGB;
use crate::hsl::HSL;
use crate::hsv::HSV;


/// Convert from `RGB` to `HSV`
///
/// **Note** floating point to unsigned integer conversions may be inaccurate due to binary to/from
/// decimal conversions
impl From<RGB> for HSV {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::rgb::RGB;
    /// use color_operators::hsv::HSV;
    ///
    /// let rgb = RGB::new(255, 0, 0);
    /// let hsv = HSV::from(rgb);
    ///
    /// assert_eq!(hsv.get("hue"), Ok(0.0));
    /// assert_eq!(hsv.get("saturation"), Ok(1.0));
    /// assert_eq!(hsv.get("value"), Ok(1.0));
    /// ```
    fn from(rgb: RGB) -> Self {
        Self::from(HSL::from(rgb))
    }
}

