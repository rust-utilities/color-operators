#!/usr/bin/env rust
#![deny(clippy::all, missing_docs, unsafe_code)]
#![allow(clippy::upper_case_acronyms, clippy::from_over_into, clippy::match_like_matches_macro)]


//! A library for converting, comparing, and preforming arithmetic on colors
//!
//! # Example
//!
//! ```rust
//! use color_operators::color::Color;
//! use color_operators::hsl::HSL;
//! use color_operators::hsv::HSV;
//! use color_operators::rgb::RGB;
//!
//! let c_rgb = Color::new_rgb(255, 42, 90);
//!
//! let hsl = HSL::from_json_string(r#"{
//!     "hue": 346.47887323943667,
//!     "saturation": 1.0,
//!     "lightness": 0.5823529411764706
//! }"#);
//!
//! let hsv = HSV::new(346.47887323943667, 0.8352941176470587, 1.0);
//!
//! assert_eq!(c_rgb, hsl);
//! assert_eq!(c_rgb, hsv);
//! assert_eq!(hsl, hsv);
//! ```


/// Enumerable for currently supported color data structures
///
/// # Example
///
/// ```rust
/// use color_operators::color::Color;
/// use color_operators::rgb::RGB;
///
/// let c_rgb = Color::new_rgb(255, 42, 90);
///
/// assert!(c_rgb.is_rgb());
///
/// let rgb: RGB = c_rgb.into();
///
/// assert_eq!(rgb.get("red"), Ok(255));
/// assert_eq!(rgb.get("green"), Ok(42));
/// assert_eq!(rgb.get("blue"), Ok(90));
/// ```
pub mod color;


/// Data structure for Hue, Saturation, Lightness encoded colors
///
/// # Example
///
/// ```rust
/// use color_operators::hsl::HSL;
///
/// let hsl = HSL::new(120.0, 1.0, 0.5);
///
/// assert_eq!(hsl.get("hue"), Ok(120.0));
/// assert_eq!(hsl.get("saturation"), Ok(1.0));
/// assert_eq!(hsl.get("lightness"), Ok(0.5));
/// ```
pub mod hsl;


/// Data structure for Hue, Saturation, Value encoded colors
///
/// # Example
///
/// ```rust
/// use color_operators::hsv::HSV;
///
/// let hsv = HSV::new(120.0, 1.0, 0.5);
///
/// assert_eq!(hsv.get("hue"), Ok(120.0));
/// assert_eq!(hsv.get("saturation"), Ok(1.0));
/// assert_eq!(hsv.get("value"), Ok(0.5));
/// ```
pub mod hsv;


/// Data structure for Red, Green, Blue encoded colors
///
/// # Example
///
/// ```rust
/// use color_operators::rgb::RGB;
///
/// let rgb = RGB::new(255, 42, 90);
///
/// assert_eq!(rgb.get("red"), Ok(255));
/// assert_eq!(rgb.get("green"), Ok(42));
/// assert_eq!(rgb.get("blue"), Ok(90));
/// ```
pub mod rgb;

