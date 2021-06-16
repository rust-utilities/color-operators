#!/usr/bin/env rust


//! TODO: Add example tests


use crate::hsl::HSL;
use crate::hsv::HSV;
use crate::rgb::RGB;


/// Adds color components for enumerable data structures
mod add;

/// Subtracts color components for enumerable data structures
mod subtract;

/// Equal and not-equal checks for enumerable data structures
mod equality;

/// Converts from `HSL` data structure into `Color::HSL`
mod convert_hsl;

/// Converts from `HSV` data structure into `Color::HSV`
mod convert_hsv;

/// Converts from `RGB` data structure into `Color::RGB`
mod convert_rgb;


/// Contains instances of supported color data structures
#[derive(Clone, Debug)]
pub enum Color {
    /// Instance of `HSL` data structure
    HSL(HSL),

    /// Instance of `HSV` data structure
    HSV(HSV),

    /// Instance of `RGB` data structure
    RGB(RGB),
}


impl Color {
    /// Initializes and warps instance of `HSL` within `Color`
    ///
    /// # Example
    ///
    /// ```rust
    /// use color_operators::color::Color;
    /// use color_operators::hsl::HSL;
    ///
    /// let c = Color::new_hsl(120.0, 0.5, 1.0);
    /// let hsl: HSL = c.clone().into();
    ///
    /// assert_eq!(c, hsl);
    /// ```
    pub fn new_hsl<T>(hue: T, saturation: T, lightness: T) -> Self
    where
        T: Into<f64>
    {
        Self::HSL(HSL::new(hue, saturation, lightness))
    }

    /// Initializes and warps instance of `HSV` within `Color`
    ///
    /// # Example
    ///
    /// ```rust
    /// use color_operators::color::Color;
    /// use color_operators::hsv::HSV;
    ///
    /// let c = Color::new_hsv(120.0, 0.5, 1.0);
    /// let hsv: HSV = c.clone().into();
    ///
    /// assert_eq!(c, hsv);
    /// ```
    pub fn new_hsv<T>(hue: T, saturation: T, value: T) -> Self
    where
        T: Into<f64>
    {
        Self::HSV(HSV::new(hue, saturation, value))
    }

    /// Initializes and warps instance of `RGB` within `Color`
    ///
    /// # Example
    ///
    /// ```rust
    /// use color_operators::color::Color;
    /// use color_operators::rgb::RGB;
    ///
    /// let c = Color::new_rgb(255, 42, 90);
    /// let rgb: RGB = c.clone().into();
    ///
    /// assert_eq!(c, rgb);
    /// ```
    pub fn new_rgb<T>(red: T, green: T, blue: T) -> Self
    where
        T: Into<u8>
    {
        Self::RGB(RGB::new(red, green, blue))
    }

    /// Attempts to rotate hue by some amount of degrees
    ///
    /// **Note** this method uses `HSL::rotate_hue` or `HSV::rotate_hue` internally
    pub fn rotate_hue<T>(&self, amount: T) -> Self
    where
        T: Into<f64>
    {
        match self {
            Color::HSL(v) => Color::HSL(v.rotate_hue(amount)),
            Color::HSV(v) => Color::HSV(v.rotate_hue(amount)),
            Color::RGB(v) => Color::RGB(v.rotate_hue(amount)),
        }
    }

    /// Attempts to this color with another
    ///
    /// **Note** wraps on overflow values
    pub fn rotate_rgb(&self, other: Self) -> Self {
        match self {
            Color::HSL(v) => Color::HSL(v.rotate_rgb(other.into())),
            Color::HSV(v) => Color::HSV(v.rotate_rgb(other.into())),
            Color::RGB(v) => Color::RGB(v.rotate_rgb(other.into())),
        }
    }

    /// Check if `Color` contains a `HSL` data structure
    ///
    /// # Example
    ///
    /// ```rust
    /// use color_operators::color::Color;
    ///
    /// let c = Color::new_hsl(120.0, 0.5, 1.0);
    /// assert!(c.is_hsl());
    /// ```
    pub fn is_hsl(&self) -> bool {
        match self {
            Color::HSL(_) => true,
            _ => false,
        }
    }

    /// Check if `Color` contains a `HSV` data structure
    ///
    /// # Example
    ///
    /// ```rust
    /// use color_operators::color::Color;
    ///
    /// let c = Color::new_hsv(120.0, 0.5, 1.0);
    /// assert!(c.is_hsv());
    /// ```
    pub fn is_hsv(&self) -> bool {
        match self {
            Color::HSV(_) => true,
            _ => false,
        }
    }

    /// Check if `Color` contains a `RGB` data structure
    ///
    /// # Example
    ///
    /// ```rust
    /// use color_operators::color::Color;
    ///
    /// let c = Color::new_rgb(255, 42, 90);
    /// assert!(c.is_rgb());
    /// ```
    pub fn is_rgb(&self) -> bool {
        match self {
            Color::RGB(_) => true,
            _ => false,
        }
    }
}

