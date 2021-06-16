#!/usr/bin/env rust


use color_operators::hsv::HSV;


#[test]
fn array_to_tuple() {
    let array = [ 13.521126760563382, 0.8352941176470587, 1.0 ];
    let hsv = HSV::from(array.clone());
    let ( hue, saturation, value ) = hsv.into();
    assert_eq!(hue, array[0]);
    assert_eq!(saturation, array[1]);
    assert_eq!(value, array[2]);
}

