#!/usr/bin/env rust


use crate::hsv::HSV;


/// Convert from vector of 64-bit precision floating point numbers
impl From<Vec<f64>> for HSV {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsv::HSV;
    ///
    /// let vector = vec![0.0, 1.0, 0.5823529411764706];
    /// let hsv = HSV::from(vector);
    ///
    /// assert_eq!(hsv.get("hue"), Ok(0.0));
    /// assert_eq!(hsv.get("saturation"), Ok(1.0));
    /// assert_eq!(hsv.get("value"), Ok(0.5823529411764706));
    /// ```
    fn from(vector: Vec<f64>) -> Self {
        let hue = vector[0].min(360.0).max(0.0);
        let saturation = vector[1].min(1.0).max(0.0);
        let value = vector[2].min(1.0).max(0.0);
        Self { hue, saturation, value }
    }
}


/// Convert into vector of 64-bit precision floating point numbers
impl Into<Vec<f64>> for HSV {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsv::HSV;
    ///
    /// let hsv = HSV::new(0.0, 1.0, 0.5823529411764706);
    /// let vector: Vec<f64> = hsv.into();
    ///
    /// assert_eq!(vector[0], 0.0);
    /// assert_eq!(vector[1], 1.0);
    /// assert_eq!(vector[2], 0.5823529411764706);
    /// ```
    fn into(self) -> Vec<f64> {
        vec![ self.hue, self.saturation, self.value ]
    }
}

