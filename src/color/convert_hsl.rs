#!/usr/bin/env rust


use crate::color::Color;
use crate::hsl::HSL;


/// Convert from `HSL` to `Color::HSL`
impl From<HSL> for Color {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::color::Color;
    /// use color_operators::hsl::HSL;
    ///
    /// let hsl = HSL::new(120.0, 0.5, 1.0);
    /// let c = Color::from(hsl);
    ///
    /// assert!(c.is_hsl());
    /// ```
    fn from(hsl: HSL) -> Self {
        Self::HSL(hsl)
    }
}

