# hips-lib

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/Kaesebrot84/hips-lib/build.yml?style=flat-square)](https://github.com/Kaesebrot84/hips-lib/actions/workflows/build.yml)
[![Crates.io](https://img.shields.io/crates/v/hips-lib?style=flat-square)](https://crates.io/crates/hips-lib)
[![Crates.io](https://img.shields.io/docsrs/hips-lib/0.2.0?style=flat-square)](https://docs.rs/hips-lib/0.2.0/hips_lib/)


Performs text to image steganography by hinding and retrieving secret text within images or pixel arrays. This is achieved by encoding the secret in the least significant bits of the R, G, B values within the image.


## Usage

Include `hips-lib` in your `Cargo.toml`:

```
hips-lib = "0.2.0"
```

Hide a secret in a vector of pixels:

```rust
use hips_lib::{color::Color, hips::{hide_secret_col, find_secret_col}};

fn main() {
    // Create a vector of four hundred pixels.
    let mut pixels = vec![Color::new(); 400];
    let password = String::from("password");

    // Your secret text
    let secret = String::from("Lorem ipsum dolor sit amet, consectetur adipisici elit, sed eiusmod tempor incidunt ut labore et dolore magna aliqua.");

    // Hide the secret in the pixel vector.
    hide_secret_col(&mut pixels, &secret, Some(password.to_owned())).unwrap();

    // Try to read the secret back from the pixel vector.
    let result = find_secret_col(&pixels, Some(password));
}
```

## Images

Include the `image` feature in your `Cargo.toml` reference.


```toml
hips-lib = { version = "0.2.0", features = ["image"]}
```

```rust
use hips_lib::hips::{find_secret_img, hide_secret_img};

fn main() {
    let secret = String::from("Lorem ipsum");
    let password = String::from("password");

    // Hide secret in target image
    let result_img = hide_secret_img("test_images/peppers.png", &secret, Some(password));

    // Find secret in another image
    let password = String::from("password");
    let result = find_secret_img("test_images/image_with_secret_password.png", Some(password)).unwrap();
}
```