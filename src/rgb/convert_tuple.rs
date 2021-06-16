#!/usr/bin/env rust


use crate::rgb::RGB;


/// Convert from tuple of unsigned 8-bit integers
impl From<(u8, u8, u8)> for RGB {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::rgb::RGB;
    ///
    /// let tuple = ( 255, 42, 90 );
    /// let rgb = RGB::from(tuple);
    ///
    /// assert_eq!(rgb.get("red"), Ok(255));
    /// assert_eq!(rgb.get("green"), Ok(42));
    /// assert_eq!(rgb.get("blue"), Ok(90));
    /// ```
    fn from(tuple: (u8, u8, u8)) -> Self {
        let (red, green, blue) = tuple;
        Self { red, green, blue }
    }
}


/// Convert into tuple of unsigned 8-bit integers
impl Into<(u8, u8, u8)> for RGB {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::rgb::RGB;
    ///
    /// let rgb = RGB::new(255, 42, 90);
    /// let ( red, green, blue ) = rgb.into();
    ///
    /// assert_eq!(red, 255);
    /// assert_eq!(green, 42);
    /// assert_eq!(blue, 90);
    /// ```
    fn into(self) -> (u8, u8, u8) {
        ( self.red, self.green, self.blue )
    }
}

