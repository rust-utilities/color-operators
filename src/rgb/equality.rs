#!/usr/bin/env rust


use crate::color::Color;
use crate::hsl::HSL;
use crate::hsv::HSV;
use crate::rgb::RGB;


impl Eq for RGB {}
impl PartialEq for RGB {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::rgb::RGB;
    ///
    /// let left = RGB::new(255, 42, 90);
    /// let right = RGB::new(255, 42, 90);
    ///
    /// assert_eq!(left, right);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        if self.red != other.red || self.green != other.green || self.blue != other.blue {
            return false;
        }
        true
    }
}


/// Converts right side from `Color` to `RGB` prior to checking equality
impl PartialEq<Color> for RGB {
    fn eq(&self, other: &Color) -> bool {
        match other {
            Color::HSL(v) => self == v,
            Color::HSV(v) => self == v,
            Color::RGB(v) => self == v,
        }
    }
}


/// Converts right side from `HSL` to `RGB` prior to checking equality
impl PartialEq<HSL> for RGB {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    /// use color_operators::rgb::RGB;
    ///
    /// let rgb = RGB::new(255, 42, 90);
    /// let hsl = HSL::from(rgb.clone());
    ///
    /// assert!(rgb == hsl);
    /// ```
    fn eq(&self, other: &HSL) -> bool {
        let other: RGB = other.clone().into();
        self == &other
    }
}


/// Converts right side from `HSV` to `RGB` prior to checking equality
impl PartialEq<HSV> for RGB {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsv::HSV;
    /// use color_operators::rgb::RGB;
    ///
    /// let rgb = RGB::new(255, 42, 90);
    /// let hsv = HSV::from(rgb.clone());
    ///
    /// assert!(rgb == hsv);
    /// ```
    fn eq(&self, other: &HSV) -> bool {
        let other: RGB = other.clone().into();
        self == &other
    }
}

