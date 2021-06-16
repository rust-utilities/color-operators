#!/usr/bin/env rust


use color_operators::hsl::HSL;
use color_operators::rgb::RGB;


#[test]
fn red_with_blue() {
    let left = HSL::from(RGB::new(24, 0, 0));
    let right = HSL::from(RGB::new(0, 0, 24));
    println!("left -> {:?}", left);
    println!("right -> {:?}", right);
    let result = left.rotate_rgb(right);
    let expected = HSL::from(RGB::new(24, 0, 24));
    assert_eq!(result, expected);
}


#[test]
fn overflow() {
    let left = HSL::from(RGB::new(250, 0, 0));
    let right = HSL::from(RGB::new(48, 0, 0));
    let result = left.rotate_rgb(right);
    let expected = HSL::from(RGB::new(43, 0, 0));
    assert_eq!(result, expected);
}

