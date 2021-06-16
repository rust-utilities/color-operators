#!/usr/bin/env rust


use color_operators::rgb::RGB;


#[test]
fn red() {
    let left = RGB::new(25, 0, 0);
    let right = RGB::new(25, 0, 0);
    let result = left - right;
    let expected = RGB::new(0, 0, 0);
    assert_eq!(result, expected);
}


#[test]
fn green() {
    let left = RGB::new(0, 25, 0);
    let right = RGB::new(0, 25, 0);
    let result = left - right;
    let expected = RGB::new(0, 0, 0);
    assert_eq!(result, expected);
}


#[test]
fn blue() {
    let left = RGB::new(0, 0, 25);
    let right = RGB::new(0, 0, 25);
    let result = left - right;
    let expected = RGB::new(0, 0, 0);
    assert_eq!(result, expected);
}


#[test]
fn all() {
    let left = RGB::new(25, 25, 25);
    let right = RGB::new(25, 25, 25);
    let result = left - right;
    let expected = RGB::new(0, 0, 0);
    assert_eq!(result, expected);
}


