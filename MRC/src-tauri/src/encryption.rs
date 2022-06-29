use magic_crypt::{new_magic_crypt, MagicCryptTrait, MagicCrypt256};

fn generate_key(password: &str) -> MagicCrypt256 {
  new_magic_crypt!(password, 256)
}

pub fn encrypt(text: &str, password: &str) -> String {
    let key = generate_key(password);
    key.encrypt_str_to_base64(text)
}

pub fn decrypt(ciphertext: &str, password: &str) -> String {
    let key = generate_key(password);
    match key.decrypt_base64_to_string(ciphertext) {
        Ok(s) => s,
        Err(_) => "".to_string(),
    }
}

#[test]
fn test_encryption() {
    let password: String = "very strong password".to_string();
    let text: String = "hi this is a text message".to_string();
    let ciphertext: String = encrypt(&text, &password);
    let decrypted: String = decrypt(&ciphertext, &password);

    assert_eq!(&text, &decrypted);
}
