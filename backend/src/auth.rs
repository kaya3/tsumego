use argon2::Argon2;
use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};

use crate::result::Result;

/// Checks a password against a stored password hash, returning `true` if the
/// password is correct and `false` otherwise.
/// 
/// Returns an error if the stored hash is invalid.
pub fn check_password(stored_hash: &str, given_password: &str) -> Result<bool> {
    let hash = PasswordHash::new(stored_hash)?;
    
    let algs: &[&dyn PasswordVerifier] = &[&Argon2::default()];
    match hash.verify_password(algs, given_password) {
        Ok(()) => Ok(true),
        Err(password_hash::Error::Password) => Ok(false),
        // This should never happen
        Err(err) => Err(err.into()),
    }
}

/// Computes a password hash for the given password, which can be stored in the
/// database. A strong password hashing algorithm with a salt is used.
/// 
/// This function cannot be used to compare a password against a stored hash;
/// instead, use the `check_password` function.
pub fn generate_password_hash(new_password: &str) -> Result<String> {
    let salt = SaltString::generate(rand::thread_rng());
    
    let hash = Argon2::default()
        .hash_password(new_password.as_bytes(), &salt)?
        .to_string();
    
    Ok(hash)
}

#[cfg(test)]
mod test {
    use super::{generate_password_hash, check_password};

    #[test]
    fn test_password_hash() {
        let hash = generate_password_hash("example").unwrap();
        
        assert_eq!(true, check_password(hash.as_str(), "example").unwrap());
        assert_eq!(false, check_password(hash.as_str(), "something else").unwrap());
    }
}
