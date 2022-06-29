use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng};
use argon2::Argon2;

pub fn hash_password(password: &str) -> String {
  let salt: SaltString = SaltString::generate(&mut OsRng);
  let argon2: Argon2 = Argon2::default();
  match argon2.hash_password(password.as_ref(), &salt) {
    Ok(hash) => hash.to_string(),
    Err(why) => {
      dbg!(password);
      dbg!(why);
      "".to_string()
    },
  }
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
  let argon2: Argon2 = Argon2::default();
  let parsed_hash: PasswordHash = match PasswordHash::new(password_hash) {
    Ok(hash) => hash,
    Err(_) => return false,
  };
  argon2.verify_password(password.as_ref(), &parsed_hash).is_ok()
}
