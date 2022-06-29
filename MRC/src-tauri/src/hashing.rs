use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng};
use argon2::Argon2;

/// Hashes the password.
///
/// # Arguments
///
/// * `password`: password to hash.
///
/// returns: String
pub fn hash_password(password: &str) -> String {
  let salt: SaltString = SaltString::generate(&mut OsRng);
  println!("{}", salt);
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

/// Verifies whether the password corresponds to the hashed password.
///
/// # Arguments
///
/// * `password`: password to verify (given by the user).
/// * `hashed_password`: password to verify withe (retrieved from database).
///
/// returns: bool
pub fn verify_password(password: &str, hashed_password: &str) -> bool {
  let argon2: Argon2 = Argon2::default();
  let parsed_hash: PasswordHash = match PasswordHash::new(hashed_password) {
    Ok(hash) => hash,
    Err(_) => return false,
  };
  argon2.verify_password(password.as_ref(), &parsed_hash).is_ok()
}

#[test]
fn test_hashing() {
  let password: &str = "password123";
  let hashed_password: String = hash_password(password);

  assert!(verify_password(password, &hashed_password));
}
