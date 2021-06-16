#!/usr/bin/env rust


use crate::hsl::HSL;
use crate::color::Color;


impl From<Color> for HSL {
    fn from(color: Color) -> Self {
        match color {
            Color::HSL(v) => v,
            Color::HSV(v) => Self::from(v),
            Color::RGB(v) => Self::from(v),
        }
    }
}

