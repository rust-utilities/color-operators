#!/usr/bin/env rust


use color_operators::rgb::RGB;


#[test]
fn vector_to_tuple() {
    let vector = vec![255, 0, 42];
    let rgb = RGB::from(vector.clone());
    let ( red, green, blue ) = rgb.into();
    assert_eq!(red, vector[0]);
    assert_eq!(green, vector[1]);
    assert_eq!(blue, vector[2]);
}

