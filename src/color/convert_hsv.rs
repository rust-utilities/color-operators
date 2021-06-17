#!/usr/bin/env rust


//! TODO: Add example tests


use crate::color::Color;
use crate::hsv::HSV;


/// Convert from `HSV` to `Color::HSV`
impl From<HSV> for Color {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::color::Color;
    /// use color_operators::hsv::HSV;
    ///
    /// let hsv = HSV::new(120.0, 0.5, 1.0);
    /// let c = Color::from(hsv);
    ///
    /// assert!(c.is_hsv());
    /// ```
    fn from(hsv: HSV) -> Self {
        Self::HSV(hsv)
    }
}

