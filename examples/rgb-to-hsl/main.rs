#!/usr/bin/env rust


extern crate argparse;
use argparse::{ ArgumentParser, StoreOption, StoreTrue };


extern crate json;


extern crate color_operators;
use color_operators::rgb::RGB;
use color_operators::hsl::HSL;


fn main() {
    let mut red_argument: Option<u8> = None;
    let mut green_argument: Option<u8> = None;
    let mut blue_argument: Option<u8> = None;
    let mut from_hex: Option<String> = None;
    let mut to_hex = false;
    let mut from_json: Option<String> = None;
    let mut to_json = false;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Convert RGB to HSL representation");

        ap.refer(&mut red_argument).add_option(
            &["--red", "-r"],
            StoreOption,
            "Red component"
        );

        ap.refer(&mut green_argument).add_option(
            &["--green", "-g"],
            StoreOption,
            "Green component"
        );

        ap.refer(&mut blue_argument).add_option(
            &["--blue", "-b"],
            StoreOption,
            "Blue component"
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

    let red = red_argument.unwrap_or_default();
    let green = green_argument.unwrap_or_default();
    let blue = blue_argument.unwrap_or_default();

    let mut rgb = RGB::new(red, green, blue);

    if let Some(hex) = from_hex {
        rgb = RGB::from_hex_string(&hex);
    } else if let Some(json_string) = from_json {
        if let Ok(json_data) = json::parse(&json_string) {
            rgb = RGB::from(json_data);
        }
    }

    let hsl = HSL::from(rgb);
    if to_hex {
        println!("{:?}", hsl.to_hex_string());
    } else if to_json {
        println!("{:?}", hsl.to_json_string());
    } else {
        println!("{:?}", hsl);
    }
}

