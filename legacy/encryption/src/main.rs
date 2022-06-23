use magic_crypt::{new_magic_crypt, MagicCryptTrait, MagicCrypt256};

fn create_key(password: &str) -> MagicCrypt256 {
    new_magic_crypt!(password, 256)
}

fn encrypt(text: &str, key: &MagicCrypt256) -> String {
    key.encrypt_str_to_base64(text)
}

fn decrypt(ciphertext: &str, key: &MagicCrypt256) -> String {
    key.decrypt_base64_to_string(ciphertext).unwrap()
}

fn main() {
    let password = "very strong password";
    let key = create_key(password);
    let text = "hi this is a text message";
    let ciphertext = encrypt(&text, &key);
    let decrypted = decrypt(&ciphertext, &key);

    println!("text: {}", text);
    println!("ciphertext: {}", ciphertext);
    println!("decrypted: {}", decrypted);
}
