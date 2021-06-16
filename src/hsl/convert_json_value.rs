#!/usr/bin/env rust


extern crate json;
use json::JsonValue;


use crate::hsl::HSL;


/// Converts from `JsonValue` key value pares
impl From<JsonValue> for HSL {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    /// extern crate json;
    ///
    /// let data = json::parse(r#"{
    ///     "hue": 50.0,
    ///     "saturation": 1.0,
    ///     "lightness": 0.5
    /// }"#).unwrap();
    ///
    /// let hsl = HSL::from(data);
    ///
    /// assert_eq!(hsl.get("hue"), Ok(50.0));
    /// assert_eq!(hsl.get("saturation"), Ok(1.0));
    /// assert_eq!(hsl.get("lightness"), Ok(0.5));
    /// ```
    fn from(data: JsonValue) -> Self {
        let hue = data["hue"].as_f64().unwrap_or_default();
        let saturation = data["saturation"].as_f64().unwrap_or_default();
        let lightness = data["lightness"].as_f64().unwrap_or_default();
        Self { hue, saturation, lightness }
    }
}

/// Converts to `JsonValue`
impl Into<JsonValue> for HSL {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsl::HSL;
    /// extern crate json;
    /// use json::JsonValue;
    ///
    /// let hsl = HSL::new(50.0, 1.0, 0.5);
    /// let data: JsonValue = hsl.into();
    ///
    /// let expected = json::parse(r#"{
    ///     "hue": 50.0,
    ///     "saturation": 1.0,
    ///     "lightness": 0.5
    /// }"#).unwrap();
    ///
    /// assert_eq!(data, expected);
    /// ```
    fn into(self) -> JsonValue {
        json::object!{
            "hue" => self.hue,
            "saturation" => self.saturation,
            "lightness" => self.lightness,
        }
    }
}

