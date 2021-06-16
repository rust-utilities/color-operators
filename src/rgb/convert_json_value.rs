#!/usr/bin/env rust


extern crate json;
use json::JsonValue;


use crate::rgb::RGB;


/// Converts from `JsonValue` key value pares
impl From<JsonValue> for RGB {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::rgb::RGB;
    /// extern crate json;
    ///
    /// let data = json::parse(r#"{
    ///     "red": 255,
    ///     "green": 42,
    ///     "blue": 90
    /// }"#).unwrap();
    ///
    /// let rgb = RGB::from(data);
    ///
    /// assert_eq!(rgb.get("red"), Ok(255));
    /// assert_eq!(rgb.get("green"), Ok(42));
    /// assert_eq!(rgb.get("blue"), Ok(90));
    /// ```
    fn from(data: JsonValue) -> Self {
        let red = data["red"].as_u8().unwrap_or_default();
        let green = data["green"].as_u8().unwrap_or_default();
        let blue = data["blue"].as_u8().unwrap_or_default();
        Self { red, green, blue }
    }
}


/// Converts to `JsonValue`
impl Into<JsonValue> for RGB {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::rgb::RGB;
    /// extern crate json;
    /// use json::JsonValue;
    ///
    /// let rgb = RGB::new(255, 42, 90);
    /// let data: JsonValue = rgb.into();
    ///
    /// let expected = json::parse(r#"{
    ///     "red": 255,
    ///     "green": 42,
    ///     "blue": 90
    /// }"#).unwrap();
    ///
    /// assert_eq!(data, expected);
    /// ```
    fn into(self) -> JsonValue {
        json::object!{
            "red" => self.red,
            "green" => self.green,
            "blue" => self.blue,
        }
    }
}

