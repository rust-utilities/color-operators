#!/usr/bin/env rust


//! TODO: Add example tests


use crate::color::Color;
use crate::rgb::RGB;


impl From<RGB> for Color {
    fn from(rgb: RGB) -> Self {
        Self::RGB(rgb)
    }
}

