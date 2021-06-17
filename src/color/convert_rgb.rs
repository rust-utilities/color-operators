#!/usr/bin/env rust


use crate::color::Color;
use crate::rgb::RGB;


/// Convert from `HSL` to `Color::HSL`
impl From<RGB> for Color {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::color::Color;
    /// use color_operators::rgb::RGB;
    ///
    /// let rgb = RGB::new(255, 42, 90);
    /// let c = Color::from(rgb);
    ///
    /// assert!(c.is_rgb());
    /// ```
    fn from(rgb: RGB) -> Self {
        Self::RGB(rgb)
    }
}

