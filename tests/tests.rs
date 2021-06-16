#!/usr/bin/env rust



mod hsl {
    mod add;
    mod rotate_rgb;
    mod subtract;

    mod from {
        mod array;
        mod hex;
        mod hsv;
        mod rgb;
        mod vector;
    }
}


mod hsv {
    mod add;
    mod rotate_rgb;
    mod subtract;

    mod from {
        mod array;
        mod hex;
        mod hsl;
        mod rgb;
        mod vector;
    }
}


mod rgb {
    mod add;
    mod rotate_rgb;
    mod subtract;

    mod from {
        mod array;
        mod hex;
        mod hsl;
        mod hsv;
        mod vector;
    }
}

