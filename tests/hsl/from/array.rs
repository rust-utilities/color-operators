#!/usr/bin/env rust


use color_operators::hsl::HSL;


#[test]
fn array_to_tuple() {
    let array = [200.0, 0.5, 0.0];
    let hsl = HSL::from(array.clone());
    let ( hue, saturation, lightness ) = hsl.into();
    assert_eq!(hue, array[0]);
    assert_eq!(saturation, array[1]);
    assert_eq!(lightness, array[2]);
}

