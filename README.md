# CroftSoft Sycamore Prototype

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/david-wallace-croft/sycamore-prototype/blob/main/LICENSE.txt

- A prototype application using the Rust-based [Sycamore](https://github.com/sycamore-rs/sycamore) front-end framework

## Demonstration

- https://www.croftsoft.com/portfolio/sycamore-prototype/

## Usage

- cd sycamore-prototype
- cargo install trunk
- trunk serve --open --public-url my-folder/sycamore-prototype

## Deploy

- trunk build --release --public-url my-folder/sycamore-prototype
- rm ../my-website-project/public_html/my-folder/sycamore-prototype/*
- cp dist/* ../my-website-project/public_html/my-folder/sycamore-prototype/

## History

- Initial release: 2022-08-13
