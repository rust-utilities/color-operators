#!/usr/bin/env rust


use crate::hsl::HSL;
use crate::rgb::RGB;


/// Convert from `HSL` to `RGB`
///
/// **Note** floating point to unsigned integer conversions may be inaccurate due to binary to/from
/// decimal conversions
impl From<HSL> for RGB {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::rgb::RGB;
    /// use color_operators::hsl::HSL;
    ///
    /// let hsl = HSL::new(0.0, 1.0, 0.5);
    /// let rgb = RGB::from(hsl);
    ///
    /// assert_eq!(rgb.get("red"), Ok(255));
    /// assert_eq!(rgb.get("green"), Ok(0));
    /// assert_eq!(rgb.get("blue"), Ok(0));
    /// ```
    fn from(hsl: HSL) -> Self {
        let ( hue, saturation, lightness ) = hsl.into();
        if hue.is_nan() {
            return Self::from(vec![0, 0, 0]);
        }

        let chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
        let hp = hue / 60.0;
        let x = chroma * (1.0 - ((hp % 2.0) - 1.0).abs());

        let mut values_f64 = vec![0.0, 0.0, 0.0];
        if hp <= 1.0 {
            values_f64 = vec![chroma, x, 0.0];
        } else if hp <= 2.0 {
            values_f64 = vec![x, chroma, 0.0];
        } else if hp <= 3.0 {
            values_f64 = vec![0.0, chroma, x];
        } else if hp <= 4.0 {
            values_f64 = vec![0.0, x, chroma];
        } else if hp <= 5.0 {
            values_f64 = vec![x, 0.0, chroma];
        } else if hp <= 6.0 {
            values_f64 = vec![chroma, 0.0, x];
        }

        let m = lightness - chroma * 0.5;

        let values_u8: Vec<u8> = values_f64.iter().map(|v| {
            (255.0 * (v + m)).round() as u8
        }).collect();

        Self::from(values_u8)
    }
}

