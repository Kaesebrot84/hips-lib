pub fn encrypt_text(secret: &str, password: &str) -> String {
    if password.is_empty() {
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
    fn encrypt_text_ut() {
        // Empty password returns secret in plain text
        let secret = "Secret";
        let password = "";

        let result = encrypt_text(secret, password);
        assert_eq!(result, secret);
        assert_eq!(result.len(), secret.len());

        // Encryption with minimal password length
        let secret = "Secret";
        let password = "a";

        let result = encrypt_text(secret, password);
        assert_ne!(result, secret);
        assert_eq!(result.len(), secret.len());
    }
}
