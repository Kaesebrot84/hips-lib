use crate::bit_ops::{BitBuffer, BitOps};
use crate::color::Color;

#[cfg(feature = "image")]
use image::GenericImage;
#[cfg(feature = "image")]
use image::{DynamicImage, GenericImageView};

#[cfg(feature = "image")]
/// Tries to load the target image and hide the given secret in it.
///
/// # Arguments
///
/// * `img_path` - Path to the target image file.
/// * `secret` - Secret text which will be hidden in the image.
///
pub fn hide_secret_img(img_path: &str, secret: &String) -> Result<DynamicImage, String> {
    if let Ok(mut img) = image::open(img_path) {
        match encode_secret_img(&mut img, secret) {
            Ok(()) => Ok(img),
            Err(err) => Err(err),
        }
    } else {
        let error = format!("Failed loading input image '{img_path}'");
        Err(error)
    }
}

#[cfg(feature = "image")]
/// Hides a secret string in the target image.
///
/// # Arguments
///
/// * `img` - Target source image the secret will be written to.
/// * `secret`  - Secret string which will be hidden in the target image.
///
fn encode_secret_img(img: &mut DynamicImage, secret: &String) -> Result<(), String> {
    if secret.is_empty() {
        return Err(String::from("You have entered an empty secret. Try to use at least one character in the secret text."));
    }

    if (img.pixels().count()) < secret.len() * 3 {
        return Err(String::from("The message is too long to be hidden in this picture. Try using a shorter message or a larger input image."));
    }

    let secret_bytes = secret.to_owned().into_bytes();

    for (byte_idx, byte) in secret_bytes.iter().enumerate() {
        // Index of the first (of three) Pixel
        let n_rgb = 3 * byte_idx as u32;

        let bit_buffer = byte.to_bit_buffer();

        for (bit_chunk_idx, bit_chunk) in bit_buffer.chunks(3).enumerate() {
            // Calculate (image) row and column of the current pixel
            let n: u32 = n_rgb + bit_chunk_idx as u32;
            let col_idx = n / img.width();
            let row_idx = n % img.width();

            // Replace the least signbificant bits for R and G values of the Pixel.
            let o_pixel = img.get_pixel(row_idx, col_idx);
            let r_out = o_pixel[0].set_lsb(bit_chunk[0]);
            let g_out = o_pixel[1].set_lsb(bit_chunk[1]);

            let b_out: u8 = if bit_chunk.len() > 2 {
                // More than 2 bits in chunk, not the last chunk. We expect more chunks to come
                o_pixel[2].set_lsb(bit_chunk[2])
            } else if byte_idx + 1 == secret_bytes.len() {
                // This is the last secret char, we set the last byte to be even.
                o_pixel[2].set_lsb(false)
            } else {
                o_pixel[2].set_lsb(true) // Last chunk but not the last secret char, we set the last byte to be odd.
            };

            // Write pixel back to image
            img.put_pixel(
                row_idx,
                col_idx,
                image::Rgba([r_out, g_out, b_out, o_pixel[3]]),
            );
        }
    }

    Ok(())
}

/// Hides a secret in an vector of pixels.
///
/// # Arguments
///
/// * `pixels` - Vector of pixels the secret will be hidden in.
/// * `secret` - The secret string.
///
pub fn hide_secret_col(pixels: &mut Vec<Color>, secret: &String) -> Result<(), String> {
    if secret.is_empty() {
        return Err(String::from("You have entered an empty secret. Try to use at least one character in the secret text."));
    }

    if pixels.len() < secret.len() * 3 {
        return Err(String::from("The message is too long to be hidden in the given pixel vector. Try using a shorter secret or a larger pixel vector."));
    }

    let secret_bytes = secret.to_owned().into_bytes();

    for (byte_idx, byte) in secret_bytes.iter().enumerate() {
        // Index of the first (of three) Pixel
        let n_rgb = 3 * byte_idx;

        let bit_buffer = byte.to_bit_buffer();

        for (bit_chunk_idx, bit_chunk) in bit_buffer.chunks(3).enumerate() {
            // Calculate (image) row and column of the current pixel
            let n: usize = n_rgb + bit_chunk_idx;

            // Replace the least signbificant bits for R and G values of the Pixel.
            let o_pixel = &pixels[n];
            let r_out = o_pixel.r.set_lsb(bit_chunk[0]);
            let g_out = o_pixel.g.set_lsb(bit_chunk[1]);

            let b_out: u8 = if bit_chunk.len() > 2 {
                // More than 2 bits in chunk, not the last chunk. We expect more chunks to come
                o_pixel.b.set_lsb(bit_chunk[2])
            } else if byte_idx + 1 == secret_bytes.len() {
                // This is the last secret char, we set the last byte to be even.
                o_pixel.b.set_lsb(false)
            } else {
                o_pixel.b.set_lsb(true) // Last chunk but not the last secret char, we set the last byte to be odd.
            };

            pixels[n] = Color::from_rgb(r_out, g_out, b_out);
        }
    }

    Ok(())
}

/// Returns a secret string retrieved from the provided pixel vector if it exists.
///
/// # Arguments
///
/// * `pixels` - Vector of pixels which will be searched for a secret string.
///
pub fn find_secret_col(pixels: &Vec<Color>) -> Option<String> {
    if pixels.len() < 3 {
        return None;
    }

    let mut result: String = String::new();
    let mut byte_array = vec![];

    // Iterate chunks of three pixels
    for pixel_chunk in pixels.chunks(3) {
        let mut byte: u8 = 0;

        // Construct a byte from three pixels
        byte = byte.set_bit(0, pixel_chunk[0].r.get_lsb());
        byte = byte.set_bit(1, pixel_chunk[0].g.get_lsb());
        byte = byte.set_bit(2, pixel_chunk[0].b.get_lsb());
        byte = byte.set_bit(3, pixel_chunk[1].r.get_lsb());
        byte = byte.set_bit(4, pixel_chunk[1].g.get_lsb());
        byte = byte.set_bit(5, pixel_chunk[1].b.get_lsb());
        byte = byte.set_bit(6, pixel_chunk[2].r.get_lsb());
        byte = byte.set_bit(7, pixel_chunk[2].g.get_lsb());

        byte_array.push(byte);

        // Last byte (blue) value of the last pixel in the chunk is even. This is the termination flag.
        if !pixel_chunk[2].b.get_lsb() {
            break;
        }
    }

    // Try convert the byte array to (secret) string
    match String::from_utf8(byte_array) {
        Ok(res) => result = format!("{result}{res}"),
        Err(_err) => return None,
    }

    if result.is_empty() || result == "\0" {
        return None;
    }

    Some(result)
}

#[cfg(feature = "image")]
/// Tries to load the target image and searches it for hidden secrets.
///
/// # Arguments
///
/// * `img_path` - Path to the image which will be searched for hidden secrets.
///
pub fn find_secret_img(img_path: &str) -> Result<Option<String>, String> {
    if let Ok(img) = image::open(img_path) {
        Ok(decode_secret_img(&img))
    } else {
        let error = format!("Failed loading input image '{img_path}'");
        Err(error)
    }
}

#[cfg(feature = "image")]
/// Returns a secret string retrieved from the provided image if it exists.
///
/// # Arguments
///
/// * `img` - Image from which a secret will be retrieved.
///
fn decode_secret_img(img: &DynamicImage) -> Option<String> {
    if img.pixels().count() < 3 {
        return None;
    }

    let mut result: String = String::new();
    let mut byte_array = vec![];

    let pixels: Vec<_> = img.pixels().collect();

    // Iterate chunks of three pixels
    for pixel_chunk in pixels.chunks(3) {
        let mut byte: u8 = 0;

        // Construct a byte from three pixels
        byte = byte.set_bit(0, pixel_chunk[0].2 .0[0].get_lsb());
        byte = byte.set_bit(1, pixel_chunk[0].2 .0[1].get_lsb());
        byte = byte.set_bit(2, pixel_chunk[0].2 .0[2].get_lsb());
        byte = byte.set_bit(3, pixel_chunk[1].2 .0[0].get_lsb());
        byte = byte.set_bit(4, pixel_chunk[1].2 .0[1].get_lsb());
        byte = byte.set_bit(5, pixel_chunk[1].2 .0[2].get_lsb());
        byte = byte.set_bit(6, pixel_chunk[2].2 .0[0].get_lsb());
        byte = byte.set_bit(7, pixel_chunk[2].2 .0[1].get_lsb());

        byte_array.push(byte);

        // Last byte (blue) value of the last pixel in the chunk is even. This is the termination flag.
        if !pixel_chunk[2].2 .0[2].get_lsb() {
            break;
        }
    }

    // Try convert the byte array to (secret) string
    match String::from_utf8(byte_array) {
        Ok(res) => result = format!("{result}{res}"),
        Err(_err) => return None,
    }

    if result.is_empty() || result == "\0" {
        return None;
    }

    Some(result)
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn encode_decode_secret_col_ut() {
        let mut pixels = vec![Color::new(); 30];

        // Byte vector with no secret returning None
        assert_eq!(None, find_secret_col(&pixels));

        // Successfully encode and decode a valid secret
        let secret = String::from("0123456789");
        let result = hide_secret_col(&mut pixels, &secret);
        assert!(result.is_ok());
        assert_eq!(Some(secret.to_owned()), find_secret_col(&pixels));

        // Return Error/None for byte vectors which cannot hold any secrets.
        let mut pixels = vec![Color::new(); 1];
        assert!(hide_secret_col(&mut pixels, &secret).is_err());
        assert_eq!(None, find_secret_col(&pixels));

        let mut pixels = vec![Color::new(); 5];
        // Return Error when secret is too long for given byte vector
        assert!(hide_secret_col(&mut pixels, &String::from("ab")).is_err());

        // Successfully encode decode the minimum size image secret combination.
        assert!(hide_secret_col(&mut pixels, &String::from("a")).is_ok());
        assert_eq!(Some(String::from("a")), find_secret_col(&pixels));

        // Providing an empty secret returns Error
        let mut pixels = vec![Color::new(); 30];
        assert!(hide_secret_col(&mut pixels, &String::from("")).is_err());
    }

    #[test]
    fn encode_decode_secret_img_ut() {
        let mut image = image::open("test_images/peppers.png").unwrap();

        // Image with no secret returning None
        assert_eq!(None, decode_secret_img(&image));

        // Successfully encode and decode a valid secret
        let secret = String::from("0123456789");
        let result = encode_secret_img(&mut image, &secret);
        assert!(result.is_ok());
        assert_eq!(Some(secret.to_owned()), decode_secret_img(&image));

        // Return Error/None for images which cannot hold any secrets.
        let mut image = image::open("test_images/1x1.png").unwrap();
        assert!(encode_secret_img(&mut image, &secret).is_err());
        assert_eq!(None, decode_secret_img(&image));

        let mut image = image::open("test_images/rgb.jpg").unwrap();
        // Return Error when secret is too long for given image
        assert!(encode_secret_img(&mut image, &String::from("ab")).is_err());

        // Successfully encode decode the minimum size image secret combination.
        assert!(encode_secret_img(&mut image, &String::from("a")).is_ok());
        assert_eq!(Some(String::from("a")), decode_secret_img(&image));

        // Providing an empty secret returns Error
        let mut image = image::open("test_images/rgb.jpg").unwrap();
        assert!(encode_secret_img(&mut image, &String::from("")).is_err());
    }

    #[test]
    fn hide_secret_img_ut() {
        // Trying to hide a secret in a non non existent image returns error
        let secret = String::from("Lorem Ipsum");
        let result = hide_secret_img("test_images/non_existent_image", &secret);
        assert!(result.is_err());

        // Successfully hide a secret in a valid image returns Ok
        let result = hide_secret_img("test_images/peppers.png", &secret);
        assert!(result.is_ok());

        // Hiding a secret in an image which is too small, returns error
        let result = hide_secret_img("test_images/rgb.jpg", &String::from("ab"));
        assert!(result.is_err());

        // Providing an empty secret returns Error
        let result = hide_secret_img("test_images/rgb.jpg", &String::from(""));
        assert!(result.is_err());
    }

    #[test]
    fn find_secret_img_ut() {
        // Image with no secret returning Error
        let result = find_secret_img("test_images/non_existent_image.png");
        assert!(result.is_err());

        // Successfully find a secret in an image
        let result = find_secret_img("test_images/image_with_secret.png");
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());

        // Try to search an image which is too small to hold secrets
        let result = find_secret_img("test_images/1x1.png");
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
