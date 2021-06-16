#!/usr/bin/env rust


use std::fmt;
use std::fmt::{Display, Formatter};


extern crate hex;


use crate::hsl::HSL;


/// Adds color components for `RGB` data structures
mod add;

/// Subtracts color components for `RGB` data structures
mod subtract;

/// Equal and not-equal checks for `RGB` data structures
mod equality;

/// Converts from/to array for `RGB` data structures
mod convert_array;

/// Converts from/to `Color` enum
mod convert_color;

/// Converts from `HSL` to `RGB` data structure
mod convert_hsl;

/// Converts from `HSV` to `RGB` data structure
mod convert_hsv;

/// Converts from/to `JsonValue` for `RGB` data structures
mod convert_json_value;

/// Converts from/to tuple for `RGB` data structures
mod convert_tuple;

/// Converts from/to vector for `RGB` data structures
mod convert_vector;


/// Data structure for Red, Green, Blue encoded colors
#[derive(Clone, Debug, Default)]
pub struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}


impl RGB {
    /// Returns new instance of `RGB` data structure
    ///
    /// # Example
    ///
    /// ```rust
    /// use color_operators::rgb::RGB;
    ///
    /// let color = RGB::new(255, 0, 0);
    ///
    /// assert_eq!(color.get("red"), Ok(255));
    /// ```
    pub fn new<T>(red: T, green: T, blue: T) -> Self
    where
        T: Into<u8>
    {
        Self { red: red.into(), green: green.into(), blue: blue.into() }
    }

    /// Returns named component value or error
    ///
    /// ```rust
    /// use color_operators::rgb::RGB;
    ///
    /// let color = RGB::new(255, 0, 0);
    ///
    /// assert_eq!(color.get("nothing"), Err("No color component named -> nothing".to_string()));
    /// ```
    pub fn get<S>(&self, component: S) -> Result<u8, String>
    where
        S: Into<String>
    {
        let component: String = component.into();
        match component.as_str() {
            "red" => Ok(self.red),
            "green" => Ok(self.green),
            "blue" => Ok(self.blue),
            _ => Err(format!("No color component named -> {}", component)),
        }
    }

    /// Returns parsed JSON string for color key/value pares, or defaults values
    ///
    /// # Example
    ///
    /// ```rust
    /// use color_operators::rgb::RGB;
    ///
    /// let rgb = RGB::from_json_string(r#"{ "red": 255, "green": 42, "blue": 90 }"#);
    ///
    /// assert_eq!(rgb.get("red"), Ok(255));
    /// assert_eq!(rgb.get("green"), Ok(42));
    /// assert_eq!(rgb.get("blue"), Ok(90));
    /// ```
    pub fn from_json_string<S>(string: S) -> Self
    where
        S: Into<String>
    {
        match json::parse(&string.into()) {
            Ok(data) => Self::from(data),
            Err(e) => {
                println!("Warning: ignoring error -> {:?}", e);
                Self { red: 0, green: 0, blue: 0 }
            }
        }
    }

    /// Serializes data structure as JSON string
    ///
    /// # Example
    ///
    /// ```rust
    /// use color_operators::rgb::RGB;
    ///
    /// let rgb = RGB::new(255, 42, 90);
    /// let data = rgb.to_json_string();
    ///
    /// let object = json::object!{
    ///     "red" => 255,
    ///     "green" => 42,
    ///     "blue" => 90
    /// };
    ///
    /// let expected = json::stringify(object);
    ///
    /// assert_eq!(data, expected);
    /// ```
    pub fn to_json_string(&self) -> String {
        json::stringify(self.clone())
    }

    /// Converts hexadecimal string into `RGB`
    ///
    /// **Warning** may panic due to unwrapping `hex::decode`
    ///
    /// # Example
    ///
    /// ```rust
    /// use color_operators::rgb::RGB;
    ///
    /// let hex = "ABCDEF";
    /// let rgb = RGB::from_hex_string(hex);
    ///
    /// assert_eq!(rgb, RGB::new(171, 205, 239));
    /// ```
    pub fn from_hex_string<S>(input: S) -> Self
    where
        S: Into<String>
    {
        let s: String = input.into();
        let values = hex::decode(s).unwrap();
        Self::from(values)
    }

    /// Returns hexadecimal string representation of `RGB` values
    ///
    /// # Example
    ///
    /// ```rust
    /// use color_operators::rgb::RGB;
    ///
    /// let color = RGB::new(255, 255, 255);
    /// let hex = color.to_hex_string();
    ///
    /// assert_eq!(hex, "FFFFFF");
    /// ```
    pub fn to_hex_string(&self) -> String {
        let vector: Vec<u8> = self.clone().into();
        vector.iter().map(|v| { format!("{:X?}", v) }).collect::<String>()
    }

    /// Attempts to rotate hue by some amount of degrees
    ///
    /// **Note** this method uses `HSL::rotate_hue` internally
    pub fn rotate_hue<T>(&self, amount: T) -> Self
    where
        T: Into<f64>
    {
        let hsl = HSL::from(self.clone());
        Self::from(hsl.rotate_hue(amount.into()))
    }

    /// Attempts to this color with another `RGB` value
    ///
    /// **Note** wraps on overflow values
    ///
    /// # Example
    ///
    /// ```rust
    /// use color_operators::rgb::RGB;
    ///
    /// let red = RGB::new(25, 0, 0);
    /// let blue = RGB::new(0, 0, 25);
    /// let result = red.rotate_rgb(blue);
    /// let expected = RGB::new(25, 0, 25);
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn rotate_rgb(&self, other: Self) -> Self {
        let left_u16: Vec<u16> = self.clone().into();
        let right_u16: Vec<u16> = other.into();

        let result = left_u16.iter().zip(right_u16.iter()).map(|(left, right)| {
            let sum = left + right;
            if sum > 255 {
                return (sum - 255) as u8;
            }
            sum as u8
        }).collect::<Vec<u8>>();

        Self::from(result)
    }
}


impl Display for RGB {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "red: {}, green: {}, blue: {}", self.red, self.green, self.blue)
    }
}

