# hips-lib

Performs text to image steganography by hinding and retrieving secret text within images or pixel arrays. This is achieved by encoding the secret in the least significant bits of the R, G, B values within the image.


## Usage

Include `hips-lib` in your `Cargo.toml`:

```
hips-lib = "0.1.0"
```

Hide a secret in a vector of pixels:

```rust
use hips_lib::{ color::Color, hips::{hide_secret_col, find_secret_col}};

fn main() {
    // Create a vector of four hundred pixels.
    let mut pixels = vec![Color::new(); 400];

    // Your secret text
    let secret = String::from("Lorem ipsum dolor sit amet, consectetur adipisici elit, sed eiusmod tempor incidunt ut labore et dolore magna aliqua.");

    // Hide the secret in the pixel vector.
    hide_secret_col(&mut pixels, &secret).unwrap();

    // Try to read the secret back from the pixel vector.
    let result = find_secret_col(&pixels);

    // Test the results
    assert_eq!(Some(secret), result);
}
```

## Images

Include the `image` feature in your `Cargo.toml` reference.


```toml
iris-lib = { version = "0.1.0", features = ["image"]}
```

```rust
use hips_lib::hips::{find_secret_img, hide_secret_img};

fn main() {
    let secret = String::from("Lorem ipsum dolor sit amet, consectetur adipisici elit, sed eiusmod tempor incidunt ut labore et dolore magna aliqua.");

    // Hide secret in target image
    let result_img = hide_secret_img("test_images/peppers.png", &secret);
    assert!(result_img.is_ok());

    // Find secret in another image
    let result = find_secret_img("test_images/image_with_secret.png").unwrap();
    assert!(result.is_some());
}
```