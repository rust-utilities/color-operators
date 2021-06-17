#!/usr/bin/env rust


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
    /// Initializes and wrapps instance of `HSL` within `Color`
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

    /// Initializes and wrapps instance of `HSV` within `Color`
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

    /// Initializes and wrapps instance of `RGB` within `Color`
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
            Self::HSL(_) => true,
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
            Self::HSV(_) => true,
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
            Self::RGB(_) => true,
            _ => false,
        }
    }

    /// Returns hexadecimal string representation of contained `Color` values
    ///
    /// # Example
    ///
    /// ```rust
    /// use color_operators::color::Color;
    ///
    /// let c_rgb = Color::new_rgb(255, 255, 255);
    /// let hex = c_rgb.to_hex_string();
    ///
    /// assert_eq!(hex, "FFFFFF");
    /// ```
    pub fn to_hex_string(&self) -> String {
        match self {
            Self::HSL(v) => v.to_hex_string(),
            Self::HSV(v) => v.to_hex_string(),
            Self::RGB(v) => v.to_hex_string(),
        }
    }

    /// Returns parsed JSON string for color key/value pares
    ///
    /// **Note** defaults to `Color::RGB` with `0` for all values
    ///
    /// # Example
    ///
    /// ```rust
    /// use color_operators::color::Color;
    ///
    /// let c_hsl = Color::from_json_string(r#"{
    ///     "hue": 120.0,
    ///     "saturation": 1.0,
    ///     "lightness": 0.75
    /// }"#);
    ///
    /// assert!(c_hsl.is_hsl());
    ///
    /// let c_hsv = Color::from_json_string(r#"{
    ///     "hue": 120.0,
    ///     "saturation": 0.5,
    ///     "value": 1.0
    /// }"#);
    ///
    /// assert!(c_hsv.is_hsv());
    ///
    /// let c_rgb = Color::from_json_string(r#"{
    ///     "red": 255,
    ///     "green": 42,
    ///     "blue": 90
    /// }"#);
    ///
    /// assert!(c_rgb.is_rgb());
    /// ```
    pub fn from_json_string<S>(string: S) -> Self
    where
        S: Into<String>
    {
        let object = match json::parse(&string.into()) {
            Ok(data) => data,
            Err(e) => {
                println!("Warning: ignoring error -> {:?}", e);
                json::object!{
                    "red": 0,
                    "green": 0,
                    "blue": 0,
                }
            }
        };

        if object.has_key("lightness") {
            Self::from(HSL::from(object))
        } else if object.has_key("value") {
            Self::from(HSV::from(object))
        } else {
            Self::from(RGB::from(object))
        }
    }

    /// Serializes contained data structure as JSON string
    ///
    /// # Example
    ///
    /// ```rust
    /// use color_operators::color::Color;
    ///
    /// let c_rgb = Color::new_rgb(255, 42, 90);
    /// let json_string = c_rgb.to_json_string();
    ///
    /// let object = json::object!{
    ///     "red" => 255,
    ///     "green" => 42,
    ///     "blue" => 90
    /// };
    ///
    /// let expected = json::stringify(object);
    ///
    /// assert_eq!(json_string, expected);
    /// ```
    pub fn to_json_string(&self) -> String {
        match self {
            Self::HSL(v) => v.to_json_string(),
            Self::HSV(v) => v.to_json_string(),
            Self::RGB(v) => v.to_json_string(),
        }
    }

    /// Attempts to rotate hue by some amount of degrees
    ///
    /// **Note** this method uses `HSL::rotate_hue` or `HSV::rotate_hue` internally
    pub fn rotate_hue<T>(&self, amount: T) -> Self
    where
        T: Into<f64>
    {
        match self {
            Self::HSL(v) => Self::HSL(v.rotate_hue(amount)),
            Self::HSV(v) => Self::HSV(v.rotate_hue(amount)),
            Self::RGB(v) => Self::RGB(v.rotate_hue(amount)),
        }
    }

    /// Attempts to rotate this color with another
    ///
    /// **Note** wraps on overflow values
    pub fn rotate_rgb(&self, other: Self) -> Self {
        match self {
            Self::HSL(v) => Self::HSL(v.rotate_rgb(other.into())),
            Self::HSV(v) => Self::HSV(v.rotate_rgb(other.into())),
            Self::RGB(v) => Self::RGB(v.rotate_rgb(other.into())),
        }
    }
}

