#!/usr/bin/env rust


use crate::hsl::HSL;
use crate::hsv::HSV;


/// Convert from `HSV` to `HSL`
impl From<HSV> for HSL {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    /// use color_operators::hsv::HSV;
    ///
    /// let hsv = HSV::new(50.0, 1.0, 1.0);
    /// let hsl = HSL::from(hsv);
    ///
    /// assert_eq!(hsl.get("hue"), Ok(50.0));
    /// assert_eq!(hsl.get("saturation"), Ok(1.0));
    /// assert_eq!(hsl.get("lightness"), Ok(0.5));
    /// ```
    fn from(hsv: HSV) -> Self {
        let ( hue, mut saturation, value ) = hsv.into();
        let lightness = value * (1.0 - (saturation / 2.0));

        saturation = 0.0;
        #[allow(clippy::float_cmp)]
        if lightness != 0.0 && lightness != 1.0 {
            saturation = (value - lightness) / lightness.min(1.0 - lightness);
        }

        Self { hue, saturation, lightness }
    }
}

