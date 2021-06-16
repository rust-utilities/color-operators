#!/usr/bin/env rust


use color_operators::hsl::HSL;
use color_operators::rgb::RGB;


#[test]
fn from_hex() {
    let hex = "ABCDEF";
    let hsl = HSL::from_hex_string(hex);
    let expected = HSL::from(RGB::new(171, 205, 239));
    assert_eq!(hsl, expected);
}

