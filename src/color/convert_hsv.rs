#!/usr/bin/env rust


//! TODO: Add example tests


use crate::color::Color;
use crate::hsv::HSV;


impl From<HSV> for Color {
    fn from(hsv: HSV) -> Self {
        Self::HSV(hsv)
    }
}

