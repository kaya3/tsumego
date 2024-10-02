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

/// The number of bytes of entropy in a session token. There is not much point
/// in this being larger than 16, since.
/// 
/// We round up to the next multiple of three, since the token is encoded in
/// base64, so each three unencoded bytes become four encoded bytes. If the
/// number of bytes is not a multiple of 3, the encoded token will end with `=`
/// characters which add no entropy.
const SESSION_TOKEN_BYTES: usize = 18;

const TOKEN_GENERATOR_MAX_RETRIES: usize = 32;

/// Generates a new random session token, and its hash. The hash should be
/// stored in the database, and the raw token should be issued to the client as
/// a cookie.
/// 
/// The `hash_is_fresh` function should check that the hash doesn't already
/// exist in the database. It is astronomically unlikely to generate duplicate
/// tokens, but this way a duplicate can be detected and regenerated.
/// 
/// Returns `(token, hash)`.
pub fn generate_session_token(hash_is_fresh: impl Fn(&str) -> bool) -> (String, String) {
    use rand::{Rng, thread_rng};
    
    let mut bytes = [0u8; SESSION_TOKEN_BYTES];
    
    let mut retries: usize = 0;
    let (raw, hash) = loop {
        thread_rng().fill(&mut bytes);
        let raw = base64_encode(&bytes);
        let hash = token_hash(raw.as_str());
        
        if hash_is_fresh(hash.as_str()) {
            break (raw, hash);
        }
        
        retries += 1;
        if retries >= TOKEN_GENERATOR_MAX_RETRIES {
            // If this ever happens, the RNG is catastrophically broken.
            // Perhaps it is returning all zeroes, or something.
            panic!("thread_rng has catastrophically low entropy");
        }
    };
    
    if retries > 0 {
        let unlikeliness = 8 * SESSION_TOKEN_BYTES * retries;
        log::warn!("Session token generated with {retries} retry(s); thread_rng probably has low entropy, as otherwise this event has probability 2^-{unlikeliness}");
    }
    
    (raw, hash)
}

/// Computes a fast hash of a session token. The hash is cryptographically
/// secure, but not suitable for passwords.
/// 
/// This function should be used to hash session tokens from client cookies,
/// in order to authenticate already-logged-in users.
pub fn token_hash(s: &str) -> String {
    use sha2::{Sha256, Digest};
    
    let mut hasher = Sha256::new();
    hasher.update(s);
    base64_encode(&hasher.finalize())
}

fn base64_encode(bytes: &[u8]) -> String {
    use base64::{engine::general_purpose::STANDARD, Engine};
    STANDARD.encode(bytes)
}

#[cfg(test)]
mod test {
    use crate::auth::token_hash;

    use super::{check_password, generate_password_hash, generate_session_token};

    #[test]
    fn test_password_hash() {
        let hash = generate_password_hash("example").unwrap();
        
        assert_eq!(true, check_password(hash.as_str(), "example").unwrap());
        assert_eq!(false, check_password(hash.as_str(), "something else").unwrap());
    }
    
    #[test]
    fn test_token_hash() {
        let (raw, hash) = generate_session_token(|_| true);
        
        assert_eq!(hash, token_hash(raw.as_str()), "Hash of raw token should equal the generated hash");
    }
    
    #[test]
    fn test_tokens_distinct() {
        let (raw1, hash1) = generate_session_token(|_| true);
        let (raw2, hash2) = generate_session_token(|h| h != hash1.as_str());
        
        assert_ne!(raw1, raw2, "Raw tokens should be distinct");
        assert_ne!(hash1, hash2, "Hashed tokens should be distinct");
    }
}
