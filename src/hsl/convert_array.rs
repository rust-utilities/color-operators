#!/usr/bin/env rust


use crate::hsl::HSL;


/// Convert from array of 64-bit precision floating point numbers
impl From<[f64; 3]> for HSL {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    ///
    /// let array = [ 0.0, 1.0, 0.5823529411764706 ];
    /// let hsl = HSL::from(array);
    ///
    /// assert_eq!(hsl.get("hue"), Ok(0.0));
    /// assert_eq!(hsl.get("saturation"), Ok(1.0));
    /// assert_eq!(hsl.get("lightness"), Ok(0.5823529411764706));
    /// ```
    fn from(array: [f64; 3]) -> Self {
        let hue = array[0].min(360.0).max(0.0);
        let saturation = array[1].min(1.0).max(0.0);
        let lightness = array[2].min(1.0).max(0.0);
        Self { hue, saturation, lightness }
    }
}

/// Convert into array of 64-bit precision floating point numbers
impl Into<[f64; 3]> for HSL {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    ///
    /// let hsl = HSL::new(0.0, 1.0, 0.5823529411764706);
    /// let array: [f64; 3] = hsl.into();
    ///
    /// assert_eq!(array[0], 0.0);
    /// assert_eq!(array[1], 1.0);
    /// assert_eq!(array[2], 0.5823529411764706);
    /// ```
    fn into(self) -> [f64; 3] {
        [ self.hue, self.saturation, self.lightness ]
    }
}

