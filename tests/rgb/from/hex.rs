#!/usr/bin/env rust


use color_operators::rgb::RGB;


#[test]
fn from_hex() {
    let hex = "ABCDEF";
    let rgb = RGB::from_hex_string(hex);
    let expected = RGB::new(171, 205, 239);
    assert_eq!(rgb, expected);
}

