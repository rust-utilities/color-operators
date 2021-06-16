#!/usr/bin/env rust


//! TODO: Add example tests


use crate::color::Color;
use crate::hsl::HSL;


impl From<HSL> for Color {
    fn from(hsl: HSL) -> Self {
        Self::HSL(hsl)
    }
}

