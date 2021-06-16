#!/usr/bin/env rust


use color_operators::rgb::RGB;


#[test]
fn red_with_blue() {
    let left = RGB::new(25, 0, 0);
    let right = RGB::new(0, 0, 25);
    let result = left.rotate_rgb(right);
    let expected = RGB::new(25, 0, 25);
    assert_eq!(result, expected);
}


#[test]
fn overflow() {
    let left = RGB::new(250, 0, 0);
    let right = RGB::new(50, 0, 0);
    let result = left.rotate_rgb(right);
    let expected = RGB::new(45, 0, 0);
    assert_eq!(result, expected);
}

