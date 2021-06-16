#!/usr/bin/env rust


use crate::rgb::RGB;
use crate::hsl::HSL;


/// Convert from `RGB` to `HSL`
///
/// **Note** floating point to unsigned integer conversions may be inaccurate due to binary to/from
/// decimal conversions
impl From<RGB> for HSL {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::rgb::RGB;
    /// use color_operators::hsl::HSL;
    ///
    /// let rgb = RGB::new(255, 0, 0);
    /// let hsl = HSL::from(rgb);
    ///
    /// assert_eq!(hsl.get("hue"), Ok(0.0));
    /// assert_eq!(hsl.get("saturation"), Ok(1.0));
    /// assert_eq!(hsl.get("lightness"), Ok(0.5));
    /// ```
    fn from(rgb: RGB) -> Self {
        let values_u8: Vec<u8> = rgb.into();
        let values_scaled: Vec<f64> = values_u8.iter().map(|v| {
            *v as f64 / 255.0
        }).collect();

        let max_scaled = *values_u8.iter().max().unwrap() as f64 / 255.0;
        let min_scaled = *values_u8.iter().min().unwrap() as f64 / 255.0;

        let difference = max_scaled - min_scaled;

        let mut hue = 0.0;
        #[allow(clippy::float_cmp)]
        if max_scaled == values_scaled[0] {
            hue = (((values_scaled[1] - values_scaled[2]) / difference % 6.0) + 6.0) % 6.0;
        } else if max_scaled == values_scaled[1] {
            hue = (values_scaled[2] - values_scaled[0]) / difference + 2.0;
        } else if max_scaled == values_scaled[2] {
            hue = (values_scaled[0] - values_scaled[1]) / difference + 4.0;
        }
        hue *= 60.0;

        let lightness = (min_scaled + max_scaled) / 2.0;
        let mut saturation = 0.0;
        if difference != 0.0 {
            saturation = difference / (1.0 - (2.0 * lightness - 1.0).abs());
        }

        Self::new(hue, saturation, lightness)
    }
}

