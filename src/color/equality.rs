#!/usr/bin/env rust


use crate::color::Color;
use crate::hsl::HSL;
use crate::hsv::HSV;
use crate::rgb::RGB;


impl Eq for Color {}
impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Color::HSL(left), Color::HSL(right)) => left == right,
            (Color::HSL(left), Color::HSV(right)) => left == right,
            (Color::HSL(left), Color::RGB(right)) => left == right,

            (Color::HSV(left), Color::HSL(right)) => left == right,
            (Color::HSV(left), Color::HSV(right)) => left == right,
            (Color::HSV(left), Color::RGB(right)) => left == right,

            (Color::RGB(left), Color::HSL(right)) => left == right,
            (Color::RGB(left), Color::HSV(right)) => left == right,
            (Color::RGB(left), Color::RGB(right)) => left == right,
        }
    }
}


impl PartialEq<HSL> for Color {
    fn eq(&self, other: &HSL) -> bool {
        match self {
            Color::HSL(v) => v == other,
            Color::HSV(v) => v == other,
            Color::RGB(v) => v == other,
        }
    }
}


impl PartialEq<HSV> for Color {
    fn eq(&self, other: &HSV) -> bool {
        match self {
            Color::HSL(v) => v == other,
            Color::HSV(v) => v == other,
            Color::RGB(v) => v == other,
        }
    }
}


impl PartialEq<RGB> for Color {
    fn eq(&self, other: &RGB) -> bool {
        match self {
            Color::HSL(v) => v == other,
            Color::HSV(v) => v == other,
            Color::RGB(v) => v == other,
        }
    }
}

