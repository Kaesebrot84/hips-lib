use hips_lib::{color::Color, hips::{hide_secret_col, find_secret_col}};

extern crate hips_lib;


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

    // Test the results
    assert_eq!(Some(secret), result);
}