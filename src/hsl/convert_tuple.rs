#!/usr/bin/env rust


use crate::hsl::HSL;


/// Convert from tuple of 64-bit precision floating point numbers
impl From <(f64, f64, f64)> for HSL {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    ///
    /// let tuple = ( 0.0, 1.0, 0.5823529411764706 );
    /// let hsl = HSL::from(tuple);
    ///
    /// assert_eq!(hsl.get("hue"), Ok(0.0));
    /// assert_eq!(hsl.get("saturation"), Ok(1.0));
    /// assert_eq!(hsl.get("lightness"), Ok(0.5823529411764706));
    /// ```
    fn from(tuple: (f64, f64, f64)) -> Self {
        let (mut hue, mut saturation, mut lightness) = tuple;
        hue = hue.min(360.0).max(0.0);
        saturation = saturation.min(1.0).max(0.0);
        lightness = lightness.min(1.0).max(0.0);
        Self { hue, saturation, lightness }
    }
}


/// Convert into vector of 64-bit precision floating point numbers
impl Into<(f64, f64, f64)> for HSL {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    ///
    /// let hsl = HSL::new(0.0, 1.0, 0.5823529411764706);
    /// let ( hue, saturation, lightness ) = hsl.into();
    ///
    /// assert_eq!(hue, 0.0);
    /// assert_eq!(saturation, 1.0);
    /// assert_eq!(lightness, 0.5823529411764706);
    /// ```
    fn into(self) -> (f64, f64, f64) {
        ( self.hue, self.saturation, self.lightness )
    }
}

