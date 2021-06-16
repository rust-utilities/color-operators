#!/usr/bin/env rust


use crate::hsl::HSL;
use crate::hsv::HSV;


/// Convert from `HSL` to `HSV`
///
/// **Note** floating point to unsigned integer conversions may be inaccurate due to binary to/from
/// decimal conversions
impl From<HSL> for HSV {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    /// use color_operators::hsv::HSV;
    ///
    /// let hsl = HSL::new(0.0, 1.0, 0.5);
    /// let hsv = HSV::from(hsl);
    ///
    /// assert_eq!(hsv.get("hue"), Ok(0.0));
    /// assert_eq!(hsv.get("saturation"), Ok(1.0));
    /// assert_eq!(hsv.get("value"), Ok(1.0));
    /// ```
    fn from(hsl: HSL) -> Self {
        let ( hue, mut saturation, lightness ) = hsl.into();

        let min = lightness.min(1.0 - lightness);
        let value = lightness + saturation * min;

        saturation = 0.0;
        if value != 0.0 {
            saturation = 2.0 * (1.0 - (lightness / value));
        }
        Self { hue, saturation, value }
    }
}

