#!/usr/bin/env rust


use std::ops::Sub;


use crate::color::Color;
use crate::hsl::HSL;
use crate::hsv::HSV;
use crate::rgb::RGB;


/// Adds color components
///
/// **Note** this uses `RGB::sub` internally
impl Sub for Color {
    type Output = Self;

    /// # Example
    ///
    /// ```rust
    /// use color_operators::color::Color;
    /// use color_operators::hsl::HSL;
    /// use color_operators::rgb::RGB;
    ///
    /// let left = Color::new_hsl(0.0, 1.0, 0.047058823529411764);
    /// let right = Color::new_hsv(0.0, 1.0, 0.09411764705882353);
    ///
    /// let result: RGB = (left - right).into();
    ///
    /// let expected = RGB::new(0, 0, 0);
    /// assert_eq!(result, expected);
    /// ```
    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Color::HSL(left), Color::HSL(right)) => Self::HSL(left - right),
            (Color::HSL(left), Color::HSV(right)) => Self::HSL(left - right),
            (Color::HSL(left), Color::RGB(right)) => Self::HSL(left - right),

            (Color::HSV(left), Color::HSL(right)) => Self::HSV(left - right),
            (Color::HSV(left), Color::HSV(right)) => Self::HSV(left - right),
            (Color::HSV(left), Color::RGB(right)) => Self::HSV(left - right),

            (Color::RGB(left), Color::HSL(right)) => Self::RGB(left - right),
            (Color::RGB(left), Color::HSV(right)) => Self::RGB(left - right),
            (Color::RGB(left), Color::RGB(right)) => Self::RGB(left - right),
        }
    }
}


/// Converts right side from `HSL` to `Color` prior to arithmetic
impl Sub<HSL> for Color {
    type Output = Self;

    /// # Example
    ///
    /// ```rust
    /// use color_operators::color::Color;
    /// use color_operators::hsl::HSL;
    /// use color_operators::rgb::RGB;
    ///
    /// let left = Color::new_rgb(24, 0, 0);
    /// let right = HSL::new(0.0, 1.0, 0.047058823529411764);
    ///
    /// let result = left - right;
    ///
    /// let expected: Color = RGB::new(0, 0, 0).into();
    /// assert_eq!(result, expected);
    /// ```
    fn sub(self, other: HSL) -> Self::Output {
        match self {
            Color::HSL(v) => Self::HSL(v - other),
            Color::HSV(v) => Self::HSV(v - other),
            Color::RGB(v) => Self::RGB(v - other),
        }
    }
}


/// Converts right side from `HSV` to `Color` prior to arithmetic
impl Sub<HSV> for Color {
    type Output = Self;

    /// # Example
    ///
    /// ```rust
    /// use color_operators::color::Color;
    /// use color_operators::hsv::HSV;
    /// use color_operators::rgb::RGB;
    ///
    /// let left = Color::new_rgb(24, 0, 0);
    /// let right = HSV::new(0.0, 1.0, 0.09411764705882353);
    ///
    /// let result = left - right;
    ///
    /// let expected: Color = RGB::new(0, 0, 0).into();
    /// assert_eq!(result, expected);
    /// ```
    fn sub(self, other: HSV) -> Self::Output {
        match self {
            Color::HSL(v) => Self::HSL(v - other),
            Color::HSV(v) => Self::HSV(v - other),
            Color::RGB(v) => Self::RGB(v - other),
        }
    }
}


/// Converts right side from `RGB` to `Color` prior to arithmetic
impl Sub<RGB> for Color {
    type Output = Self;

    /// # Example
    ///
    /// ```rust
    /// use color_operators::color::Color;
    /// use color_operators::rgb::RGB;
    ///
    /// let left = Color::new_rgb(24, 0, 0);
    /// let right = RGB::new(24, 0, 0);
    ///
    /// let result = left - right;
    ///
    /// let expected: Color = RGB::new(0, 0, 0).into();
    /// assert_eq!(result, expected);
    /// ```
    fn sub(self, other: RGB) -> Self::Output {
        match self {
            Color::HSL(v) => Self::HSL(v - other),
            Color::HSV(v) => Self::HSV(v - other),
            Color::RGB(v) => Self::RGB(v - other),
        }
    }
}

