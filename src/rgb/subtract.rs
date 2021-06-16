#!/usr/bin/env rust


use std::ops::Sub;


use crate::color::Color;
use crate::hsl::HSL;
use crate::hsv::HSV;
use crate::rgb::RGB;


/// Subtracts color components
impl Sub for RGB {
    type Output = Self;

    /// # Example
    ///
    /// ```rust
    /// use color_operators::rgb::RGB;
    ///
    /// let left = RGB::new(155, 0, 0);
    /// let right = RGB::new(100, 0, 0);
    /// let result = left - right;
    /// let expected = RGB::new(55, 0, 0);
    ///
    /// assert_eq!(result, expected);
    /// ```
    fn sub(self, other: Self) -> Self::Output {
        let red = self.red - other.red;
        let green = self.green - other.green;
        let blue = self.blue - other.blue;
        Self { red, green, blue }
    }
}


/// Converts right side from `Color` to `RGB` prior to arithmetic
impl Sub<Color> for RGB {
    type Output = Self;

    fn sub(self, other: Color) -> Self::Output {
        match other {
            Color::HSL(v) => self - Self::from(v),
            Color::HSV(v) => self - Self::from(v),
            Color::RGB(v) => self - v,
        }
    }
}


/// Converts right side from `HSL` to `RGB` prior arithmetic
impl Sub<HSL> for RGB {
    type Output = Self;

    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    /// use color_operators::rgb::RGB;
    ///
    /// let rgb = RGB::new(56, 42, 90);
    /// let hsl = HSL::from(rgb.clone());
    ///
    /// let expected = RGB::new(0, 0, 0);
    ///
    /// assert_eq!(rgb - hsl, expected);
    /// ```
    fn sub(self, other: HSL) -> Self::Output {
        let other: RGB = other.into();
        self - other
    }
}


/// Converts right side from `HSV` to `RGB` prior arithmetic
impl Sub<HSV> for RGB {
    type Output = Self;

    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsv::HSV;
    /// use color_operators::rgb::RGB;
    ///
    /// let rgb = RGB::new(56, 42, 90);
    /// let hsv = HSV::from(rgb.clone());
    ///
    /// let expected = RGB::new(0, 0, 0);
    ///
    /// assert_eq!(rgb - hsv, expected);
    /// ```
    fn sub(self, other: HSV) -> Self::Output {
        let other: RGB = other.into();
        self - other
    }
}

