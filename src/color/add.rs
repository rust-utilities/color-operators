#!/usr/bin/env rust


//! TODO: Add example tests


use std::ops::Add;


use crate::color::Color;
use crate::hsl::HSL;
use crate::hsv::HSV;
use crate::rgb::RGB;


/// Adds color components
///
/// **Note** this uses `RGB::add` internally
impl Add for Color {
    type Output = Self;

    /// # Example
    ///
    /// ```rust
    /// use color_operators::color::Color;
    /// use color_operators::hsl::HSL;
    /// use color_operators::hsv::HSV;
    ///
    /// let left = Color::new_hsl(0.0, 1.0, 0.047058823529411764);
    /// let right = Color::new_rgb(24, 0, 0);
    ///
    /// let result: HSV = (left + right).into();
    ///
    /// let expected = HSV::new(0.0, 1.0, 0.18823529411764706);
    /// assert_eq!(result, expected);
    /// ```
    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Color::HSL(left), Color::HSL(right)) => Self::HSL(left + right),
            (Color::HSL(left), Color::HSV(right)) => Self::HSL(left + right),
            (Color::HSL(left), Color::RGB(right)) => Self::HSL(left + right),

            (Color::HSV(left), Color::HSL(right)) => Self::HSV(left + right),
            (Color::HSV(left), Color::HSV(right)) => Self::HSV(left + right),
            (Color::HSV(left), Color::RGB(right)) => Self::HSV(left + right),

            (Color::RGB(left), Color::HSL(right)) => Self::RGB(left + right),
            (Color::RGB(left), Color::HSV(right)) => Self::RGB(left + right),
            (Color::RGB(left), Color::RGB(right)) => Self::RGB(left + right),
        }
    }
}


/// Converts right side from `HSL` to `Color` prior to arithmetic
impl Add<HSL> for Color {
    type Output = Self;

    fn add(self, other: HSL) -> Self::Output {
        match self {
            Color::HSL(v) => Self::HSL(v + other),
            Color::HSV(v) => Self::HSV(v + other),
            Color::RGB(v) => Self::RGB(v + other),
        }
    }
}


/// Converts right side from `HSV` to `Color` prior to arithmetic
impl Add<HSV> for Color {
    type Output = Self;

    fn add(self, other: HSV) -> Self::Output {
        match self {
            Color::HSL(v) => Self::HSL(v + other),
            Color::HSV(v) => Self::HSV(v + other),
            Color::RGB(v) => Self::RGB(v + other),
        }
    }
}


/// Converts right side from `RGB` to `Color` prior to arithmetic
impl Add<RGB> for Color {
    type Output = Self;

    fn add(self, other: RGB) -> Self::Output {
        match self {
            Color::HSL(v) => Self::HSL(v + other),
            Color::HSV(v) => Self::HSV(v + other),
            Color::RGB(v) => Self::RGB(v + other),
        }
    }
}

