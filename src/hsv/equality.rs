#!/usr/bin/env rust


use crate::color::Color;
use crate::hsl::HSL;
use crate::hsv::HSV;
use crate::rgb::RGB;


impl Eq for HSV {}
impl PartialEq for HSV {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsv::HSV;
    ///
    /// let left = HSV::new(0.0, 0.8352941176470587, 1.0);
    /// let right = HSV::new(0.0, 0.8352941176470587, 1.0);
    ///
    /// assert_eq!(left, right);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        if self.hue != other.hue || self.saturation != other.saturation || self.value != other.value {
            return false;
        }
        true
    }
}


/// Converts right side from `Color` to `HSV` prior to checking equality
impl PartialEq<Color> for HSV {
    fn eq(&self, other: &Color) -> bool {
        match other {
            Color::HSL(v) => self == v,
            Color::HSV(v) => self == v,
            Color::RGB(v) => self == v,
        }
    }
}


/// Converts right side from `HSL` to `HSV` prior to checking equality
impl PartialEq<HSL> for HSV {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    /// use color_operators::hsv::HSV;
    ///
    /// let hsv = HSV::new(0.0, 0.8352941176470587, 1.0);
    /// let hsl = HSL::from(hsv.clone());
    ///
    /// assert!(hsv == hsl);
    /// ```
    fn eq(&self, other: &HSL) -> bool {
        let other: HSV = other.clone().into();
        self == &other
    }
}


/// Converts right side from `RGB` to `HSV` prior to checking equality
impl PartialEq<RGB> for HSV {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::rgb::RGB;
    /// use color_operators::hsv::HSV;
    ///
    /// let hsv = HSV::new(0.0, 0.8352941176470587, 1.0);
    /// let rgb = RGB::from(hsv.clone());
    ///
    /// assert!(hsv == rgb);
    /// ```
    fn eq(&self, other: &RGB) -> bool {
        let other: HSV = other.clone().into();
        self == &other
    }
}

