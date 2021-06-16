#!/usr/bin/env rust


use crate::hsv::HSV;
use crate::color::Color;


impl From<Color> for HSV {
    fn from(color: Color) -> Self {
        match color {
            Color::HSL(v) => Self::from(v),
            Color::HSV(v) => v,
            Color::RGB(v) => Self::from(v),
        }
    }
}

