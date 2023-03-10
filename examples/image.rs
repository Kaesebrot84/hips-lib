extern crate hips_lib;
use hips_lib::hips::{find_secret_img, hide_secret_img};

fn main() {
    let secret = String::from("Lorem ipsum");
    let password = String::from("password");

    // Hide secret in target image
    let result_img = hide_secret_img("test_images/peppers.png", &secret, Some(password));
    assert!(result_img.is_ok());

    // Find secret in another image
    let password = String::from("password");
    let result = find_secret_img("test_images/image_with_secret_password.png", Some(password)).unwrap();

    assert!(result.is_some());
    assert_eq!(secret, result.unwrap());
}
