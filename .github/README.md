# Color Operators
[heading__top]:
  #color-operators
  "&#x2B06; Color data structures, converters, comparators, and arithmetic operators"


Color data structures, converters, comparators, and arithmetic operators


## [![Byte size of Color Operators][badge__main__color_operators__source_code]][color_operators__main__source_code] [![Open Issues][badge__issues__color_operators]][issues__color_operators] [![Open Pull Requests][badge__pull_requests__color_operators]][pull_requests__color_operators] [![Latest commits][badge__commits__color_operators__main]][commits__color_operators__main] [![Build Status][badge_travis_ci]][build_travis_ci]


---


- [&#x2B06; Top of Document][heading__top]

- [&#x1F3D7; Requirements][heading__requirements]

- [&#9889; Quick Start][heading__quick_start]

- [&#x1F9F0; Usage][heading__usage]

- [&#x1F5D2; Notes][heading__notes]

- [&#x1F4C8; Contributing][heading__contributing]

  - [&#x1F531; Forking][heading__forking]
  - [&#x1F4B1; Sponsor][heading__sponsor]

- [&#x1F4C7; Attribution][heading__attribution]

- [&#x2696; Licensing][heading__license]


---



## Requirements
[heading__requirements]:
  #requirements
  "&#x1F3D7; Prerequisites and/or dependencies that this project needs to function properly"


This repository requires [Rust][rust_home] language/compiler to build from source


As of last update to this ReadMe file, the recommended method of installing Rust is via the installer script...


```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


______


## Quick Start
[heading__quick_start]:
  #quick-start
  "&#9889; Perhaps as easy as one, 2.0,..."


This repository is a Rust library, define it as a dependency within a project `Cargo.toml` file...


**`Cargo.toml` (snip)**


```toml
[dependencies]
color-operators = "0.0.1"
```


> Check [Rust -- Doc -- Specifying Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) for details about defining dependencies.


Then include within a source file via `use` statement...


**`src/main.rs` (snip)**


```rust
extern crate color_operators;

use color_operators::hsl::HSL;
use color_operators::hsv::HSV;
use color_operators::rgb::RGB;
```


______


## Usage
[heading__usage]:
  #usage
  "&#x1F9F0; How to utilize this repository"


Create a project if one does not already exist...


```bash
cargo init hsl-to-rgb

cd hsl-to-rgb
```


Add this crate as a dependency...


**`Cargo.toml` (snip)**


```toml
[dependencies]
color-operators = "0.0.1"
argparse = "0.2.2"
```


> Note, `argparse` is optional and only included to ensure following example functions as shown


Write code that utilizes this crate...


**`src/main.rs`**


```rust
extern crate argparse;
use argparse::{ ArgumentParser, StoreOption };


extern crate json;


extern crate color_operators;
use color_operators::rgb::RGB;
use color_operators::hsl::HSL;


fn main() {
    let mut red_argument: Option<u8> = None;
    let mut green_argument: Option<u8> = None;
    let mut blue_argument: Option<u8> = None;

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

        ap.parse_args_or_exit();
    }

    let red = red_argument.unwrap_or_default();
    let green = green_argument.unwrap_or_default();
    let blue = blue_argument.unwrap_or_default();

    let rgb = RGB::new(red, green, blue);
    let hsl = HSL::from(rgb);
    println!("{:?}", hsl);
}
```


Test conversion with various options...


```bash
cargo run -- -r 255
#> HSL { hue: 0.0, saturation: 1.0, lightness: 0.5 }

cargo run -- -g 255
#> HSL { hue: 120.0, saturation: 1.0, lightness: 0.5 }

cargo run -- -b 255
#> HSL { hue: 240.0, saturation: 1.0, lightness: 0.5 }

cargo run -- -r 42 -b 42
#> HSL { hue: 300.0, saturation: 1.0, lightness: 0.08235294117647059 }
```


______


## Notes
[heading__notes]:
  #notes
  "&#x1F5D2; Additional things to keep in mind when developing"


This repository may not be feature complete and/or fully functional, Pull Requests that add features or fix bugs are certainly welcomed.


---


**Warning** color conversion is not guaranteed to be fully reversible! For example `25 + 25` may **not** result in `50` after conversions, where as `24 + 24` will usually equal `48`... This seems to often be because of binary and decimal conversions and arithmetic.


---


Examples may be found within the `examples/` directory and run to test conversions and other features;


```bash
cargo run --example rgb-to-hsl -- -r 255
#> HSL { hue: 0.0, saturation: 1.0, lightness: 0.5 }

cargo run --example rgb-to-hsv -- -g 255
#> HSV { hue: 120.0, saturation: 1.0, value: 1.0 }
```


---


For regular operations this project depends;


- [`hex`](https://crates.io/crates/hex) crate provides hexadecimal string to decimal parsing

- [`json`](https://crates.io/crates/json) crate allows for string serialization and de-serialization


Examples depend on;


- [`argparse`](https://crates.io/crates/argparse) carte provides command line argument parsing and type coercion


Lents depend on;


- [`clippy`](https://crates.io/crates/clippy) crate throws errors and warnings for common _code-smells_


______


## Contributing
[heading__contributing]:
  #contributing
  "&#x1F4C8; Options for contributing to color-operators and rust-utilities"


Options for contributing to color-operators and rust-utilities


---


### Forking
[heading__forking]:
  #forking
  "&#x1F531; Tips for forking color-operators"


Start making a [Fork][color_operators__fork_it] of this repository to an account that you have write permissions for.


- Add remote for fork URL. The URL syntax is _`git@github.com:<NAME>/<REPO>.git`_...


```bash
cd ~/git/hub/rust-utilities/color-operators

git remote add fork git@github.com:<NAME>/color-operators.git
```


- Commit your changes and push to your fork, eg. to fix an issue...


```bash
cd ~/git/hub/rust-utilities/color-operators


git commit -F- <<'EOF'
:bug: Fixes #42 Issue


**Edits**


- `<SCRIPT-NAME>` script, fixes some bug reported in issue
EOF


git push fork main
```


> Note, the `-u` option may be used to set `fork` as the default remote, eg. _`git push -u fork main`_ however, this will also default the `fork` remote for pulling from too! Meaning that pulling updates from `origin` must be done explicitly, eg. _`git pull origin main`_


- Then on GitHub submit a Pull Request through the Web-UI, the URL syntax is _`https://github.com/<NAME>/<REPO>/pull/new/<BRANCH>`_


> Note; to decrease the chances of your Pull Request needing modifications before being accepted, please check the [dot-github](https://github.com/rust-utilities/.github) repository for detailed contributing guidelines.


**Tip** check for `TODO` lines for known arias that may cause issues, as well as portions of code that are excellent targets for new Pull Requests...


```bash
grep -C3 -ri 'todo' src/**/*.rs
```


---


### Sponsor
  [heading__sponsor]:
  #sponsor
  "&#x1F4B1; Methods for financially supporting rust-utilities that maintains color-operators"


Thanks for even considering it!


Via Liberapay you may <sub>[![sponsor__shields_io__liberapay]][sponsor__link__liberapay]</sub> on a repeating basis.


Regardless of if you're able to financially support projects such as color-operators that rust-utilities maintains, please consider sharing projects that are useful with others, because one of the goals of maintaining Open Source repositories is to provide value to the community.


______


## Attribution
[heading__attribution]:
  #attribution
  "&#x1F4C7; Resources that where helpful in building this project so far."


- [GitHub -- `github-utilities/make-readme`](https://github.com/github-utilities/make-readme)

- [StackOverflow -- How can I convert a hex string to a u8 slice?](https://stackoverflow.com/questions/52987181/)

- [StackOverflow -- Checking equality of custom structs](https://stackoverflow.com/questions/35161176/)

- [Wikipedia -- HSL and HSV](https://en.wikipedia.org/wiki/HSL_color_space)


______


## License
[heading__license]:
  #license
  "&#x2696; Legal side of Open Source"


```
Color data structures, converters, comparators, and arithmetic operators
Copyright (C) 2021 S0AndS0

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, version 3 of the License.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```


For further details review full length version of [AGPL-3.0][branch__current__license] License.



[branch__current__license]:
  /LICENSE
  "&#x2696; Full length version of AGPL-3.0 License"


[badge__commits__color_operators__main]:
  https://img.shields.io/github/last-commit/rust-utilities/color-operators/main.svg

[commits__color_operators__main]:
  https://github.com/rust-utilities/color-operators/commits/main
  "&#x1F4DD; History of changes on this branch"


[color_operators__community]:
  https://github.com/rust-utilities/color-operators/community
  "&#x1F331; Dedicated to functioning code"


[issues__color_operators]:
  https://github.com/rust-utilities/color-operators/issues
  "&#x2622; Search for and _bump_ existing issues or open new issues for project maintainer to address."

[color_operators__fork_it]:
  https://github.com/rust-utilities/color-operators/
  "&#x1F531; Fork it!"

[pull_requests__color_operators]:
  https://github.com/rust-utilities/color-operators/pulls
  "&#x1F3D7; Pull Request friendly, though please check the Community guidelines"

[color_operators__main__source_code]:
  https://github.com/rust-utilities/color-operators/
  "&#x2328; Project source!"

[badge__issues__color_operators]:
  https://img.shields.io/github/issues/rust-utilities/color-operators.svg

[badge__pull_requests__color_operators]:
  https://img.shields.io/github/issues-pr/rust-utilities/color-operators.svg

[badge__main__color_operators__source_code]:
  https://img.shields.io/github/repo-size/rust-utilities/color-operators


[rust_home]:
  https://www.rust-lang.org/
  "Home page for Rust language"

[rust_github]:
  https://github.com/rust-lang
  "Source code for Rust on GitHub"

[sponsor__shields_io__liberapay]:
  https://img.shields.io/static/v1?logo=liberapay&label=Sponsor&message=rust-utilities

[sponsor__link__liberapay]:
  https://liberapay.com/rust-utilities
  "&#x1F4B1; Sponsor developments and projects that rust-utilities maintains via Liberapay"


[badge_travis_ci]:
  https://travis-ci.com/rust-utilities/color-operators.svg?branch=main

[build_travis_ci]:
  https://travis-ci.com/rust-utilities/color-operators

