#!/usr/bin/env rust


use std::fmt;
use std::fmt::{Display, Formatter};


use crate::rgb::RGB;


/// Adds color components for `HSV` data structures
mod add;

/// Subtracts color components for `HSV` data structures
mod subtract;

/// Equal and not-equal checks for `HSV` data structures
mod equality;

/// Converts from/to array for `HSV` data structures
mod convert_array;

/// Converts from/to `Color` enum
mod convert_color;

/// Converts from/to `JsonValue` for `HSV` data structures
mod convert_json_value;

/// Converts from `HSL` to `HSV` data structure
mod convert_hsl;

/// Converts from `RGB` to `HSV` data structure
mod convert_rgb;

/// Converts from/to tuple for `HSV` data structures
mod convert_tuple;

/// Converts from/to vector for `HSV` data structures
mod convert_vector;


/// Data structure for Hue, Saturation, Value encoded colors
#[derive(Clone, Debug, Default)]
pub struct HSV {
    hue: f64,
    saturation: f64,
    value: f64,
}


impl HSV {
    /// Returns new instance of `HSV` data structure
    ///
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsv::HSV;
    ///
    /// let color = HSV::new(60.0, 1.0, 0.5);
    ///
    /// assert_eq!(color.get("hue"), Ok(60.0));
    /// assert_eq!(color.get("saturation"), Ok(1.0));
    /// assert_eq!(color.get("value"), Ok(0.5));
    /// ```
    pub fn new<T>(hue: T, saturation: T, value: T) -> Self
    where
        T: Into<f64>
    {
        let hue: f64 = hue.into().min(360.0).max(0.0);
        let saturation: f64 = saturation.into().min(1.0).max(0.0);
        let value: f64 = value.into().min(1.0).max(0.0);
        Self { hue, saturation, value }
    }

    /// Returns named component value or error
    ///
    /// ```rust
    /// use color_operators::hsv::HSV;
    ///
    /// let color = HSV::new(60.0, 1.0, 0.5);
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
            "value" => Ok(self.value),
            _ => Err(format!("No color component named -> {}", component)),
        }
    }

    /// Returns parsed JSON string for color key/value pares, or defaults values
    ///
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsv::HSV;
    ///
    /// let hsv = HSV::from_json_string(r#"{ "hue": 60.0, "saturation": 1.0, "value": 0.5 }"#);
    ///
    /// assert_eq!(hsv.get("hue"), Ok(60.0));
    /// assert_eq!(hsv.get("saturation"), Ok(1.0));
    /// assert_eq!(hsv.get("value"), Ok(0.5));
    /// ```
    pub fn from_json_string<S>(string: S) -> Self
    where
        S: Into<String>
    {
        match json::parse(&string.into()) {
            Ok(data) => Self::from(data),
            Err(e) => {
                println!("Warning: ignoring error -> {:?}", e);
                Self { hue: 0.0, saturation: 0.0, value: 0.0 }
            }
        }
    }

    /// Serializes data structure as JSON string
    ///
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsv::HSV;
    ///
    /// let hsv = HSV::new(60.0, 1.0, 0.5);
    /// let data = hsv.to_json_string();
    ///
    /// let object = json::object!{
    ///     "hue" => 60.0,
    ///     "saturation" => 1.0,
    ///     "value" => 0.5
    /// };
    ///
    /// let expected = json::stringify(object);
    ///
    /// assert_eq!(data, expected);
    /// ```
    pub fn to_json_string(&self) -> String {
        json::stringify(self.clone())
    }

    /// Converts hexadecimal string into `HSV`
    ///
    /// **Warning** this method uses `RGB::from_hex_string` and may panic
    pub fn from_hex_string<S>(input: S) -> Self
    where
        S: Into<String>
    {
        Self::from(RGB::from_hex_string(input))
    }

    /// Returns hexadecimal string representation of `HSV` values
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
    /// use color_operators::hsv::HSV;
    ///
    /// let color = HSV::new(120.0, 0.5, 1.0);
    /// let clockwise = color.rotate_hue(90.0);
    ///
    /// let expected = HSV::new(210.0, 0.5, 1.0);
    ///
    /// assert_eq!(clockwise, expected);
    /// ```
    ///
    /// Negative values rotates hue anticlockwise
    ///
    /// ```rust
    /// use color_operators::hsv::HSV;
    ///
    /// let color = HSV::new(120.0, 0.5, 1.0);
    /// let anticlockwise = color.rotate_hue(-180.0);
    ///
    /// let expected = HSV::new(300.0, 0.5, 1.0);
    ///
    /// assert_eq!(anticlockwise, expected);
    /// ```
    pub fn rotate_hue<T>(&self, amount: T) -> Self
    where
        T: Into<f64>
    {
        let mut amount: f64 = amount.into();
        let ( mut hue, saturation, value ) = self.clone().into();

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

        Self { hue, saturation, value }
    }

    /// Attempts to rotate_rgb this color with another `HSL` value
    ///
    /// **Note** this method uses `RGB::rotate_rgb` internally
    pub fn rotate_rgb(&self, other: Self) -> Self {
        let rgb_left: RGB = self.clone().into();
        let rgb_right: RGB = other.into();
        let sum = rgb_left.rotate_rgb(rgb_right);
        Self::from(sum)
    }
}


impl Display for HSV {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "hue: {}, saturation: {}, value: {}", self.hue, self.saturation, self.value)
    }
}

