#!/usr/bin/env rust


extern crate json;
use json::JsonValue;


use crate::hsv::HSV;


/// Converts from `JsonValue` key value pares
impl From<JsonValue> for HSV {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsv::HSV;
    /// extern crate json;
    ///
    /// let data = json::parse(r#"{
    ///     "hue": 120.0,
    ///     "saturation": 1.0,
    ///     "value": 1.0
    /// }"#).unwrap();
    ///
    /// let hsv = HSV::from(data);
    ///
    /// assert_eq!(hsv.get("hue"), Ok(120.0));
    /// assert_eq!(hsv.get("saturation"), Ok(1.0));
    /// assert_eq!(hsv.get("value"), Ok(1.0));
    /// ```
    fn from(data: JsonValue) -> Self {
        let hue = data["hue"].as_f64().unwrap_or_default();
        let saturation = data["saturation"].as_f64().unwrap_or_default();
        let value = data["value"].as_f64().unwrap_or_default();
        Self { hue, saturation, value }
    }
}


/// Converts to `JsonValue`
impl Into<JsonValue> for HSV {
    /// # Example
    ///
    /// ```rust
    /// use color_operators::hsv::HSV;
    /// extern crate json;
    /// use json::JsonValue;
    ///
    /// let hsv = HSV::new(120.0, 1.0, 1.0);
    /// let data: JsonValue = hsv.into();
    ///
    /// let expected = json::parse(r#"{
    ///     "hue": 120.0,
    ///     "saturation": 1.0,
    ///     "value": 1.0
    /// }"#).unwrap();
    ///
    /// assert_eq!(data, expected);
    /// ```
    fn into(self) -> JsonValue {
        json::object!{
            "hue" => self.hue,
            "saturation" => self.saturation,
            "value" => self.value,
        }
    }
}

