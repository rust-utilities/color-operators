#!/usr/bin/env rust


extern crate argparse;
use argparse::{ ArgumentParser, StoreOption, StoreTrue };


extern crate json;


extern crate color_operators;
use color_operators::rgb::RGB;
use color_operators::hsv::HSV;


fn main() {
    let mut hue_argument: Option<f64> = None;
    let mut saturation_argument: Option<f64> = None;
    let mut value_argument: Option<f64> = None;
    let mut from_hex: Option<String> = None;
    let mut to_hex = false;
    let mut from_json: Option<String> = None;
    let mut to_json = false;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Convert HSV to RGB representation");

        ap.refer(&mut hue_argument).add_option(
            &["--hue", "-h"],
            StoreOption,
            "Hue component"
        );

        ap.refer(&mut saturation_argument).add_option(
            &["--saturation", "-s"],
            StoreOption,
            "Saturation component"
        );

        ap.refer(&mut value_argument).add_option(
            &["--value", "-v"],
            StoreOption,
            "Lightness component"
        );

        ap.refer(&mut from_hex).add_option(
            &["--from-hex"],
            StoreOption,
            "Converts hexadecimal to HSV"
        );

        ap.refer(&mut to_hex).add_option(
            &["--to-hex"],
            StoreTrue,
            "Prints RGB as hexadecimal value"
        );

        ap.refer(&mut from_json).add_option(
            &["--from-json"],
            StoreOption,
            "Reads string input as JSON data"
        );

        ap.refer(&mut to_json).add_option(
            &["--to-json"],
            StoreTrue,
            "Outputs data as JSON string"
        );

        ap.parse_args_or_exit();
    }

    let hue = hue_argument.unwrap_or_default();
    let saturation = saturation_argument.unwrap_or_default();
    let value = value_argument.unwrap_or_default();

    let mut hsv = HSV::new(hue, saturation, value);

    if let Some(hex) = from_hex {
        hsv = HSV::from_hex_string(&hex);
    } else if let Some(json_string) = from_json {
        if let Ok(json_data) = json::parse(&json_string) {
            hsv = HSV::from(json_data);
        }
    }

    let rgb = RGB::from(hsv);
    if to_hex {
        println!("{:?}", rgb.to_hex_string());
    } else if to_json {
        println!("{:?}", rgb.to_json_string());
    } else {
        println!("{:?}", rgb);
    }
}

