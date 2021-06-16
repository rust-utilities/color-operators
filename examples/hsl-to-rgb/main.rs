#!/usr/bin/env rust


extern crate argparse;
use argparse::{ ArgumentParser, StoreOption, StoreTrue };


extern crate json;


extern crate color_operators;
use color_operators::rgb::RGB;
use color_operators::hsl::HSL;


fn main() {
    let mut hue_argument: Option<f64> = None;
    let mut saturation_argument: Option<f64> = None;
    let mut lightness_argument: Option<f64> = None;
    let mut from_hex: Option<String> = None;
    let mut to_hex = false;
    let mut from_json: Option<String> = None;
    let mut to_json = false;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Convert HSL to RGB representation");

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

        ap.refer(&mut lightness_argument).add_option(
            &["--lightness", "-l"],
            StoreOption,
            "Lightness component"
        );

        ap.refer(&mut from_hex).add_option(
            &["--from-hex"],
            StoreOption,
            "Converts hexadecimal to HSL"
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
    let lightness = lightness_argument.unwrap_or_default();

    let mut hsl = HSL::new(hue, saturation, lightness);

    if let Some(hex) = from_hex {
        hsl = HSL::from_hex_string(&hex);
    } else if let Some(json_string) = from_json {
        if let Ok(json_data) = json::parse(&json_string) {
            hsl = HSL::from(json_data);
        }
    }

    let rgb = RGB::from(hsl);
    if to_hex {
        println!("{:?}", rgb.to_hex_string());
    } else if to_json {
        println!("{:?}", rgb.to_json_string());
    } else {
        println!("{:?}", rgb);
    }
}

