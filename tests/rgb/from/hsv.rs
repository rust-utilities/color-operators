#!/usr/bin/env rust


use color_operators::hsv::HSV;
use color_operators::rgb::RGB;


#[test]
fn red() {
    let hsv = HSV::new(0.0, 1.0, 1.0);
    let rgb = RGB::from(hsv);
    let expected = RGB::new(255, 0, 0);
    assert_eq!(rgb, expected);
}


#[test]
fn green() {
    let hsv = HSV::new(120.0, 1.0, 1.0);
    let rgb = RGB::from(hsv);
    let expected = RGB::new(0, 255, 0);
    assert_eq!(rgb, expected);
}


#[test]
fn blue() {
    let hsv = HSV::new(240.0, 1.0, 1.0);
    let rgb = RGB::from(hsv);
    let expected = RGB::new(0, 0, 255);
    assert_eq!(rgb, expected);
}

