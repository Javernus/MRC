use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::Argon2;

pub fn hash_password(password: &str) -> String {
  // let salt = SaltString::generate(&mut OsRng);
  let argon2: Argon2 = Argon2::default();
  argon2.hash_password(password.as_ref(), "").unwrap().to_string()
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
  let argon2: Argon2 = Argon2::default();
  let parsed_hash: PasswordHash = PasswordHash::new(password_hash).unwrap();
  argon2.verify_password(password.as_ref(), &parsed_hash).is_ok()
}
