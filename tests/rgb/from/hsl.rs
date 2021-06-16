#!/usr/bin/env rust


use color_operators::hsl::HSL;
use color_operators::rgb::RGB;


#[test]
fn red() {
    let hsl = HSL::new(0.0, 1.0, 0.5);
    let rgb = RGB::from(hsl);
    let expected = RGB::new(255, 0, 0);
    assert_eq!(rgb, expected);
}


#[test]
fn green() {
    let hsl = HSL::new(120.0, 1.0, 0.5);
    let rgb = RGB::from(hsl);
    let expected = RGB::new(0, 255, 0);
    assert_eq!(rgb, expected);
}


#[test]
fn blue() {
    let hsl = HSL::new(240.0, 1.0, 0.5);
    let rgb = RGB::from(hsl);
    let expected = RGB::new(0, 0, 255);
    assert_eq!(rgb, expected);
}


#[test]
fn invariance() {
    let rgb = RGB::new(50, 50, 50);
    let hsl = HSL::from(rgb.clone());
    println!("hsl -> {}", hsl);
    let left = RGB::from(hsl);
    assert_eq!(left, rgb);
}

