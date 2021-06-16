#!/usr/bin/env rust
#![deny(clippy::all, missing_docs, unsafe_code)]


//! A library for converting, comparing, and preforming arithmetic on colors


/// Enumerable for currently supported color data structures
pub mod color;


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

