#!/usr/bin/env rust


use crate::rgb::RGB;
use crate::color::Color;


impl From<Color> for RGB {
    fn from(color: Color) -> Self {
        match color {
            Color::HSL(v) => Self::from(v),
            Color::HSV(v) => Self::from(v),
            Color::RGB(v) => v,
        }
    }
}

