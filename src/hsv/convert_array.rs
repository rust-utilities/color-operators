#!/usr/bin/env rust


use crate::hsv::HSV;


/// Convert from array of 64-bit precision floating point numbers
impl From<[f64; 3]> for HSV {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsv::HSV;
    ///
    /// let array = [ 0.0, 1.0, 0.5823529411764706 ];
    /// let hsv = HSV::from(array);
    ///
    /// assert_eq!(hsv.get("hue"), Ok(0.0));
    /// assert_eq!(hsv.get("saturation"), Ok(1.0));
    /// assert_eq!(hsv.get("value"), Ok(0.5823529411764706));
    /// ```
    fn from(array: [f64; 3]) -> Self {
        let hue = array[0].min(360.0).max(0.0);
        let saturation = array[1].min(1.0).max(0.0);
        let value = array[2].min(1.0).max(0.0);
        Self { hue, saturation, value }
    }
}


/// Convert into array of 64-bit precision floating point numbers
impl Into<[f64; 3]> for HSV {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsv::HSV;
    ///
    /// let hsv = HSV::new(0.0, 1.0, 0.5823529411764706);
    /// let array: [f64; 3] = hsv.into();
    ///
    /// assert_eq!(array[0], 0.0);
    /// assert_eq!(array[1], 1.0);
    /// assert_eq!(array[2], 0.5823529411764706);
    /// ```
    fn into(self) -> [f64; 3] {
        [ self.hue, self.saturation, self.value ]
    }
}

