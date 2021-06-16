#!/usr/bin/env rust


use crate::rgb::RGB;


/// Convert from array of unsigned 8-bit integers
impl From<[u8; 3]> for RGB {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::rgb::RGB;
    ///
    /// let array = [ 255, 42, 90 ];
    /// let rgb = RGB::from(array);
    ///
    /// assert_eq!(rgb.get("red"), Ok(255));
    /// assert_eq!(rgb.get("green"), Ok(42));
    /// assert_eq!(rgb.get("blue"), Ok(90));
    /// ```
    fn from(array: [u8; 3]) -> Self {
        let red = array[0];
        let green = array[1];
        let blue = array[2];
        Self { red, green, blue }
    }
}


/// Convert into array of unsigned 8-bit integers
impl Into<[u8; 3]> for RGB {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::rgb::RGB;
    ///
    /// let rgb = RGB::new(255, 42, 90);
    /// let array: [u8; 3] = rgb.into();
    ///
    /// assert_eq!(array[0], 255);
    /// assert_eq!(array[1], 42);
    /// assert_eq!(array[2], 90);
    /// ```
    fn into(self) -> [u8; 3] {
        [ self.red, self.green, self.blue ]
    }
}

