#!/usr/bin/env rust


use color_operators::hsv::HSV;


#[test]
fn vector_to_tuple() {
    let vector = vec![13.521126760563382, 0.8352941176470587, 1.0];
    let hsv = HSV::from(vector.clone());
    let ( hue, saturation, value ) = hsv.into();
    assert_eq!(hue, vector[0]);
    assert_eq!(saturation, vector[1]);
    assert_eq!(value, vector[2]);
}

