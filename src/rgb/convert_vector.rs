#!/usr/bin/env rust


use crate::rgb::RGB;


/// Convert from vector of unsigned 8-bit integers
impl<T> From<Vec<T>> for RGB
where
    T: Into<u8> + Copy
{
    /// # Example
    ///
    /// ```rust
    /// use color_operators::rgb::RGB;
    ///
    /// let vector = vec![255, 42, 90];
    /// let rgb = RGB::from(vector);
    ///
    /// assert_eq!(rgb.get("red"), Ok(255));
    /// assert_eq!(rgb.get("green"), Ok(42));
    /// assert_eq!(rgb.get("blue"), Ok(90));
    /// ```
    fn from(vector: Vec<T>) -> Self {
        let red = vector[0];
        let green = vector[1];
        let blue = vector[2];
        Self::new(red, green, blue)
    }
}


/// Convert into vector of unsigned 8-bit integers
impl Into<Vec<u8>> for RGB {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::rgb::RGB;
    ///
    /// let rgb = RGB::new(255, 42, 90);
    /// let vector: Vec<u8> = rgb.into();
    ///
    /// assert_eq!(vector[0], 255);
    /// assert_eq!(vector[1], 42);
    /// assert_eq!(vector[2], 90);
    /// ```
    fn into(self) -> Vec<u8> {
        vec![ self.red, self.green, self.blue ]
    }
}


/// Convert from vector of unsigned 16-bit integers
///
/// **Note** this is used internally to allow `Self::rotate_rgb` to _wrap_ instead of overflowing
impl Into<Vec<u16>> for RGB {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::rgb::RGB;
    ///
    /// let rgb = RGB::new(255, 42, 90);
    /// let vector: Vec<u16> = rgb.into();
    ///
    /// assert_eq!(vector[0], 255);
    /// assert_eq!(vector[1], 42);
    /// assert_eq!(vector[2], 90);
    /// ```
    fn into(self) -> Vec<u16> {
        let vector_u8: Vec<u8> = self.into();
        vector_u8.iter().map(|v| {
            *v as u16
        }).collect::<Vec<u16>>()
    }
}

