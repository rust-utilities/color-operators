#!/usr/bin/env rust


use crate::hsl::HSL;


/// Convert from vector of 64-bit precision floating point numbers
impl From<Vec<f64>> for HSL {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    ///
    /// let vector = vec![0.0, 1.0, 0.5823529411764706];
    /// let hsl = HSL::from(vector);
    ///
    /// assert_eq!(hsl.get("hue"), Ok(0.0));
    /// assert_eq!(hsl.get("saturation"), Ok(1.0));
    /// assert_eq!(hsl.get("lightness"), Ok(0.5823529411764706));
    /// ```
    fn from(vector: Vec<f64>) -> Self {
        let hue = vector[0].min(360.0).max(0.0);
        let saturation = vector[1].min(1.0).max(0.0);
        let lightness = vector[2].min(1.0).max(0.0);
        Self { hue, saturation, lightness }
    }
}


/// Convert into vector of 64-bit precision floating point numbers
impl Into<Vec<f64>> for HSL {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    ///
    /// let hsl = HSL::new(0.0, 1.0, 0.5823529411764706);
    /// let vector: Vec<f64> = hsl.into();
    ///
    /// assert_eq!(vector[0], 0.0);
    /// assert_eq!(vector[1], 1.0);
    /// assert_eq!(vector[2], 0.5823529411764706);
    /// ```
    fn into(self) -> Vec<f64> {
        vec![ self.hue, self.saturation, self.lightness ]
    }
}

