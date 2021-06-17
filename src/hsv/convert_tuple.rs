#!/usr/bin/env rust


use crate::hsv::HSV;


/// Convert from tuple of 64-bit precision floating point numbers
impl<T> From <(T, T, T)> for HSV
where
    T: Into<f64> + Copy
{
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsv::HSV;
    ///
    /// let tuple = ( 0.0, 1.0, 0.5823529411764706 );
    /// let hsv = HSV::from(tuple);
    ///
    /// assert_eq!(hsv.get("hue"), Ok(0.0));
    /// assert_eq!(hsv.get("saturation"), Ok(1.0));
    /// assert_eq!(hsv.get("value"), Ok(0.5823529411764706));
    /// ```
    fn from(tuple: (T, T, T)) -> Self {
        let (hue, saturation, value) = tuple;
        Self::new(hue, saturation, value)
    }
}


/// Convert into vector of 64-bit precision floating point numbers
impl Into<(f64, f64, f64)> for HSV {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsv::HSV;
    ///
    /// let hsv = HSV::new(0.0, 1.0, 0.5823529411764706);
    /// let ( hue, saturation, value ) = hsv.into();
    ///
    /// assert_eq!(hue, 0.0);
    /// assert_eq!(saturation, 1.0);
    /// assert_eq!(value, 0.5823529411764706);
    /// ```
    fn into(self) -> (f64, f64, f64) {
        ( self.hue, self.saturation, self.value )
    }
}

