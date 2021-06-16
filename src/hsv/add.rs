#!/usr/bin/env rust


use std::ops::Add;


use crate::color::Color;
use crate::hsl::HSL;
use crate::hsv::HSV;
use crate::rgb::RGB;


/// Adds color components
///
/// **Note** this uses `RGB::add` internally
impl Add for HSV {
    type Output = Self;

    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsv::HSV;
    ///
    /// let left = HSV::new(0.0, 1.0, 0.09411764705882353);
    /// let right = HSV::new(0.0, 1.0, 0.09411764705882353);
    /// let result = left + right;
    ///
    /// let expected = HSV::new(0.0, 1.0, 0.18823529411764706);
    /// assert_eq!(result, expected);
    /// ```
    fn add(self, other: Self) -> Self::Output {
        let left = RGB::from(self);
        let right = RGB::from(other);
        HSV::from(left + right)
    }
}


/// Converts right side from `Color` to `HSV` prior to arithmetic
impl Add<Color> for HSV {
    type Output = Self;

    fn add(self, other: Color) -> Self::Output {
        match other {
            Color::HSL(v) => self + Self::from(v),
            Color::HSV(v) => self + v,
            Color::RGB(v) => self + Self::from(v),
        }
    }
}


/// Converts right side from `HSL` to `HSV` prior arithmetic
impl Add<HSL> for HSV {
    type Output = Self;

    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    /// use color_operators::hsv::HSV;
    /// use color_operators::rgb::RGB;
    ///
    /// let rgb = RGB::new(56, 42, 90);
    /// let hsv = HSV::from(rgb);
    /// let hsl = HSL::from(hsv.clone());
    ///
    /// let expected = HSV::from(RGB::new(112, 84, 180));
    ///
    /// assert_eq!(hsv + hsl, expected);
    /// ```
    fn add(self, other: HSL) -> Self::Output {
        let other: HSV = other.into();
        self + other
    }
}


/// Converts right side from `RGB` to `HSV` prior arithmetic
impl Add<RGB> for HSV {
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
    /// let expected = HSV::from(RGB::new(112, 84, 180));
    ///
    /// assert_eq!(hsv + rgb, expected);
    /// ```
    fn add(self, other: RGB) -> Self::Output {
        let other: HSV = other.into();
        self + other
    }
}

