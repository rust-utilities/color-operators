#!/usr/bin/env rust


use color_operators::hsv::HSV;
use color_operators::rgb::RGB;


#[test]
fn red() {
    let rgb = RGB::new(255, 0, 0);
    let hsv = HSV::from(rgb);
    let expected = HSV::new(0.0, 1.0, 1.0);
    assert_eq!(hsv, expected);
}


#[test]
fn green() {
    let rgb = RGB::new(0, 255, 0);
    let hsv = HSV::from(rgb);
    let expected = HSV::new(120.0, 1.0, 1.0);
    assert_eq!(hsv, expected);
}


#[test]
fn blue() {
    let rgb = RGB::new(0, 0, 255);
    let hsv = HSV::from(rgb);
    let expected = HSV::new(240.0, 1.0, 1.0);
    assert_eq!(hsv, expected);
}

