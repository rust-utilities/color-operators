#!/usr/bin/env rust


use color_operators::hsv::HSV;


#[test]
fn from_hex() {
    let hex = "ABCDEF";
    let hsv = HSV::from_hex_string(hex);
    let expected = HSV::new(210.0, 0.2845188284518827, 0.9372549019607843 );
    assert_eq!(hsv, expected);
}

