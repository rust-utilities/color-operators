#!/usr/bin/env rust


use color_operators::hsl::HSL;
use color_operators::hsv::HSV;


#[test]
fn yellowish() {
    let hsv = HSV::new(50.0, 1.0, 1.0);
    let hsl = HSL::from(hsv);
    let expected = HSL::new(50.0, 1.0, 0.5);
    assert_eq!(hsl, expected);
}

