#!/usr/bin/env rust


use crate::color::Color;
use crate::hsl::HSL;
use crate::hsv::HSV;
use crate::rgb::RGB;


impl Eq for HSL {}
impl PartialEq for HSL {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    ///
    /// let left = HSL::new(0.0, 1.0, 0.5823529411764706);
    /// let right = HSL::new(0.0, 1.0, 0.5823529411764706);
    ///
    /// assert_eq!(left, right);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        if self.hue != other.hue || self.saturation != other.saturation || self.lightness != other.lightness {
            return false;
        }
        true
    }
}


/// Converts right side to `HSL` prior to checking equality
impl PartialEq<Color> for HSL {
    fn eq(&self, other: &Color) -> bool {
        match other {
            Color::HSL(v) => self == v,
            Color::HSV(v) => self == v,
            Color::RGB(v) => self == v,
        }
    }
}


/// Converts right side from `HSV` to `HSL` prior to checking equality
impl PartialEq<HSV> for HSL {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    /// use color_operators::hsv::HSV;
    ///
    /// let hsl = HSL::new(0.0, 1.0, 0.5823529411764706);
    /// let hsv = HSV::from(hsl.clone());
    ///
    /// assert!(hsl == hsv);
    /// ```
    fn eq(&self, other: &HSV) -> bool {
        let other: HSL = other.clone().into();
        self == &other
    }
}


/// Converts right side from `RGB` to `HSL` prior to checking equality
impl PartialEq<RGB> for HSL {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    /// use color_operators::rgb::RGB;
    ///
    /// let hsl = HSL::new(0.0, 1.0, 0.5823529411764706);
    /// let rgb = RGB::from(hsl.clone());
    ///
    /// assert!(hsl == rgb);
    /// ```
    fn eq(&self, other: &RGB) -> bool {
        let other: HSL = other.clone().into();
        self == &other
    }
}

