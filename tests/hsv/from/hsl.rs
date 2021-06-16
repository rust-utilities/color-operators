#!/usr/bin/env rust


use color_operators::hsl::HSL;
use color_operators::hsv::HSV;
// use color_operators::rgb::RGB;


#[test]
fn red() {
    let hsl = HSL::new(0.0, 1.0, 0.5);
    let hsv = HSV::from(hsl);
    let expected = HSV::new(0.0, 1.0, 1.0);
    assert_eq!(hsv, expected);
}


#[test]
fn green() {
    let hsl = HSL::new(120.0, 1.0, 0.5);
    let hsv = HSV::from(hsl);
    let expected = HSV::new(120.0, 1.0, 1.0);
    assert_eq!(hsv, expected);
}


#[test]
fn blue() {
    let hsl = HSL::new(240.0, 1.0, 0.5);
    let hsv = HSV::from(hsl);
    let expected = HSV::new(240.0, 1.0, 1.0);
    assert_eq!(hsv, expected);
}

