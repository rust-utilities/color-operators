#!/usr/bin/env rust


use color_operators::hsl::HSL;


#[test]
fn vector_to_tuple() {
    let vector = vec![200.0, 0.5, 0.0];
    let hsl = HSL::from(vector.clone());
    let ( hue, saturation, lightness ) = hsl.into();
    assert_eq!(hue, vector[0]);
    assert_eq!(saturation, vector[1]);
    assert_eq!(lightness, vector[2]);
}

