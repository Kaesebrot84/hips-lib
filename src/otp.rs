/// One-time pad encryption of a string with a provided password string.
/// Can be used for both encryption and decryption.
/// 
/// # Arguments
/// 
/// * `secret` - The secret string
/// * `password` - The password string
/// 
pub fn otp(secret: &str, password: &str) -> String {
    if password.is_empty() || secret.is_empty() {
        return secret.to_string();
    }

    let mut result: String = String::new();
    for (idx, sec_byte) in secret.bytes().enumerate() {
        let pw_idx = idx % password.len();
        let c = (sec_byte ^ password.bytes().nth(pw_idx).unwrap()) as char;
        result.push(c);
    }

    result
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn otp_ut() {
        // Empty secret returns empty result
        let result = otp("", "");
        assert!(result.is_empty());

        // Empty password returns secret in plain text
        let secret = "Secret";
        let password = "";

        let result = otp(secret, password);
        assert_eq!(result, secret);
        assert_eq!(result.len(), secret.len());

        // Encryption with minimal password length
        let password = "a";

        let result = otp(secret, password);
        assert_ne!(result, secret);
        assert_eq!(result.len(), secret.len());

        // Reverse way decrypt secret to plain text
        let result = otp(&result, password);
        assert_eq!(result, secret);

        // Encrypt with password longer than the secret
        let password = "Lorem Ipsum is the best ipsum.";
        let result = otp(secret, password);
        assert_ne!(secret, result);
        assert_eq!(secret.len(), result.len());

        // Decrypt with password longer than the secret
        let result = otp(&result, password);
        assert_eq!(result, secret);
    }
}
