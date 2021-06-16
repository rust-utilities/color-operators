#!/usr/bin/env rust


use std::fmt;
use std::fmt::{Display, Formatter};


use crate::rgb::RGB;


/// Adds color components for `HSL` data structures
mod add;

/// Subtracts color components for `HSL` data structures
mod subtract;

/// Equal and not-equal checks for `HSL` data structures
mod equality;

/// Converts from/to array for `HSL` data structures
mod convert_array;

/// Converts from/to `Color` enum
mod convert_color;

/// Converts from/to `JsonValue` for `RGB` data structures
mod convert_json_value;

/// Converts `HSV` to `HSL` data structure
mod convert_hsv;

/// Converts `RGB` to `HSL` data structure
mod convert_rgb;

/// Converts from/to tuple for `HSL` data structures
mod convert_tuple;

/// Converts from/to vector for `HSL` data structures
mod convert_vector;


/// Data structure for Hue, Saturation, Lightness encoded colors
#[derive(Clone, Debug, Default)]
pub struct HSL {
    hue: f64,
    saturation: f64,
    lightness: f64,
}


impl HSL {
    /// Returns new instance of `HSL` data structure
    ///
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    ///
    /// let color = HSL::new(60.0, 1.0, 0.5);
    ///
    /// assert_eq!(color.get("hue"), Ok(60.0));
    /// assert_eq!(color.get("saturation"), Ok(1.0));
    /// assert_eq!(color.get("lightness"), Ok(0.5));
    /// ```
    pub fn new<T>(hue: T, saturation: T, lightness: T) -> Self
    where
        T: Into<f64>
    {
        let hue: f64 = hue.into().min(360.0).max(0.0);
        let saturation: f64 = saturation.into().min(1.0).max(0.0);
        let lightness: f64 = lightness.into().min(1.0).max(0.0);
        Self { hue, saturation, lightness }
    }

    /// Returns named component value or error
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    ///
    /// let color = HSL::new(60.0, 1.0, 0.5);
    ///
    /// assert_eq!(color.get("nothing"), Err("No color component named -> nothing".to_string()));
    /// ```
    pub fn get<S>(&self, component: S) -> Result<f64, String>
    where
        S: Into<String>
    {
        let component: String = component.into();
        match component.as_str() {
            "hue" => Ok(self.hue),
            "saturation" => Ok(self.saturation),
            "lightness" => Ok(self.lightness),
            _ => Err(format!("No color component named -> {}", component)),
        }
    }

    /// Returns parsed JSON string for color key/value pares, or defaults values
    ///
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    ///
    /// let hsl = HSL::from_json_string(r#"{ "hue": 60.0, "saturation": 1.0, "lightness": 0.5 }"#);
    ///
    /// assert_eq!(hsl.get("hue"), Ok(60.0));
    /// assert_eq!(hsl.get("saturation"), Ok(1.0));
    /// assert_eq!(hsl.get("lightness"), Ok(0.5));
    /// ```
    pub fn from_json_string<S>(string: S) -> Self
    where
        S: Into<String>
    {
        match json::parse(&string.into()) {
            Ok(data) => Self::from(data),
            Err(e) => {
                println!("Warning: ignoring error -> {:?}", e);
                Self { hue: 0.0, saturation: 0.0, lightness: 0.0 }
            }
        }
    }

    /// Serializes data structure as JSON string
    ///
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    ///
    /// let hsl = HSL::new(60.0, 1.0, 0.5);
    /// let data = hsl.to_json_string();
    ///
    /// let object = json::object!{
    ///     "hue" => 60.0,
    ///     "saturation" => 1.0,
    ///     "lightness" => 0.5
    /// };
    ///
    /// let expected = json::stringify(object);
    ///
    /// assert_eq!(data, expected);
    /// ```
    pub fn to_json_string(&self) -> String {
        json::stringify(self.clone())
    }

    /// Converts hexadecimal string into `HSL`
    ///
    /// **Warning** this method uses `RGB::from_hex_string` and may panic
    pub fn from_hex_string<S>(input: S) -> Self
    where
        S: Into<String>
    {
        Self::from(RGB::from_hex_string(input))
    }

    /// Returns hexadecimal string representation of `HSL` values
    ///
    /// **Note** this method uses `RGB::to_hex_string` internally
    pub fn to_hex_string(&self) -> String {
        RGB::from(self.clone()).to_hex_string()
    }

    /// Attempts to rotate hue by some amount of degrees
    ///
    /// # Examples
    ///
    /// Positive values rotates hue clockwise
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    ///
    /// let color = HSL::new(120.0, 0.5, 1.0);
    /// let clockwise = color.rotate_hue(90.0);
    ///
    /// let expected = HSL::new(210.0, 0.5, 1.0);
    ///
    /// assert_eq!(clockwise, expected);
    /// ```
    ///
    /// Negative values rotates hue anticlockwise
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    ///
    /// let color = HSL::new(120.0, 0.5, 1.0);
    /// let anticlockwise = color.rotate_hue(-180.0);
    ///
    /// let expected = HSL::new(300.0, 0.5, 1.0);
    ///
    /// assert_eq!(anticlockwise, expected);
    /// ```
    pub fn rotate_hue<T>(&self, amount: T) -> Self
    where
        T: Into<f64>
    {
        let mut amount: f64 = amount.into();
        let ( mut hue, saturation, lightness ) = self.clone().into();

        let max = 360.0;
        if amount < 0.0 {
            amount += max;
        }

        let sum = hue + amount;
        if sum > max {
            hue = (sum - (max * (sum / max).round())).abs();
        } else {
            hue = sum;
        }

        Self { hue, saturation, lightness }
    }

    /// Attempts to rotate this color with another `HSL` value
    ///
    /// **Note** this method uses `RGB::rotate_rgb` internally
    pub fn rotate_rgb(&self, other: Self) -> Self {
        let rgb_left: RGB = self.clone().into();
        let rgb_right: RGB = other.into();
        let sum = rgb_left.rotate_rgb(rgb_right);
        Self::from(sum)
    }
}


impl Display for HSL {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "hue: {}, saturation: {}, lightness: {}", self.hue, self.saturation, self.lightness)
    }
}

