#!/usr/bin/env rust


use std::ops::Sub;


use crate::color::Color;
use crate::hsl::HSL;
use crate::hsv::HSV;
use crate::rgb::RGB;


/// Subtracts color components
///
/// **Note** this method uses `RGB::sub` internally
impl Sub for HSL {
    type Output = Self;

    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    ///
    /// let left = HSL::new(0.0, 1.0, 0.09411764705882353);
    /// let right = HSL::new(0.0, 1.0, 0.047058823529411764);
    /// let result = left - right;
    ///
    /// let expected = HSL::new(0.0, 1.0, 0.047058823529411764);
    /// assert_eq!(result, expected);
    /// ```
    fn sub(self, other: Self) -> Self::Output {
        let left = RGB::from(self);
        let right = RGB::from(other);
        HSL::from(left - right)
    }
}


/// Converts right side from `Color` to `HSL` prior to arithmetic
impl Sub<Color> for HSL {
    type Output = Self;

    fn sub(self, other: Color) -> Self::Output {
        match other {
            Color::HSL(v) => self - v,
            Color::HSV(v) => self - Self::from(v),
            Color::RGB(v) => self - Self::from(v),
        }
    }
}


/// Converts right side from `HSV` to `HSL` prior arithmetic
impl Sub<HSV> for HSL {
    type Output = Self;

    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    /// use color_operators::hsv::HSV;
    /// use color_operators::rgb::RGB;
    ///
    /// let rgb = RGB::new(56, 42, 90);
    /// let hsl = HSL::from(rgb);
    /// let hsv = HSV::from(hsl.clone());
    ///
    /// let expected = HSL::from(RGB::new(0, 0, 0));
    ///
    /// assert_eq!(hsl - hsv, expected);
    /// ```
    fn sub(self, other: HSV) -> Self::Output {
        let other: HSL = other.into();
        self - other
    }
}


/// Converts right side from `RGB` to `HSL` prior arithmetic
impl Sub<RGB> for HSL {
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
    /// let expected = HSL::from(RGB::new(0, 0, 0));
    ///
    /// assert_eq!(hsl - rgb, expected);
    /// ```
    fn sub(self, other: RGB) -> Self::Output {
        let other: HSL = other.into();
        self - other
    }
}

