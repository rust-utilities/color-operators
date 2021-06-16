#!/usr/bin/env rust


use color_operators::rgb::RGB;


#[test]
fn array_to_tuple() {
    let array = [ 255, 0, 42 ];
    let rgb = RGB::from(array.clone());
    let ( red, green, blue ) = rgb.into();
    assert_eq!(red, array[0]);
    assert_eq!(green, array[1]);
    assert_eq!(blue, array[2]);
}

