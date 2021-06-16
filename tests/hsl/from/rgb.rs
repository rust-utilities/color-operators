#!/usr/bin/env rust


use color_operators::hsl::HSL;
use color_operators::rgb::RGB;


#[test]
fn red() {
    let rgb = RGB::new(255, 0, 0);
    let hsl = HSL::from(rgb);
    let expected = HSL::new(0.0, 1.0, 0.5);
    assert_eq!(hsl, expected);
}


#[test]
fn green() {
    let rgb = RGB::new(0, 255, 0);
    let hsl = HSL::from(rgb);
    let expected = HSL::new(120.0, 1.0, 0.5);
    assert_eq!(hsl, expected);
}


#[test]
fn blue() {
    let rgb = RGB::new(0, 0, 255);
    let hsl = HSL::from(rgb);
    let expected = HSL::new(240.0, 1.0, 0.5);
    assert_eq!(hsl, expected);
}

