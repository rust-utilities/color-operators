#!/usr/bin/env rust


use color_operators::hsl::HSL;
use color_operators::rgb::RGB;


#[test]
fn red() {
    let left = HSL::from(RGB::new(48, 0, 0));
    let right = HSL::from(RGB::new(24, 0, 0));
    let result = left - right;
    let expected = HSL::from(RGB::new(24, 0, 0));
    assert_eq!(result, expected);
}


#[test]
fn green() {
    let left = HSL::from(RGB::new(0, 48, 0));
    let right = HSL::from(RGB::new(0, 24, 0));
    let result = left - right;
    let expected = HSL::from(RGB::new(0, 24, 0));
    assert_eq!(result, expected);
}


#[test]
fn blue() {
    let left = HSL::from(RGB::new(0, 0, 48));
    let right = HSL::from(RGB::new(0, 0, 24));
    let result = left - right;
    let expected = HSL::from(RGB::new(0, 0, 24));
    assert_eq!(result, expected);
}


