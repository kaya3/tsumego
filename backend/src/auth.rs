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
/// in this being larger than 16; OWASP recommends at least 8 bytes of entropy.
/// https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html#session-id-entropy
/// 
/// We round up to the next multiple of three, since the token is encoded in
/// base64, so each three unencoded bytes become four encoded bytes. If the
/// number of bytes is not a multiple of 3, the encoded token will end with `=`
/// characters which add no entropy.
pub const SESSION_TOKEN_BYTES: usize = 18;

/// Generates a new random session token, and its hash. The hash should be
/// stored in the database, and the raw token should be issued to the client as
/// a cookie.
/// 
/// Returns `(token, hash)`.
pub fn generate_session_token() -> (String, String) {
    use rand::{Rng, thread_rng};
    
    let mut bytes = [0u8; SESSION_TOKEN_BYTES];
    thread_rng().fill(&mut bytes);
    let raw = base64_encode(&bytes);
    let hash = token_hash(raw.as_str());
    
    (raw, hash)
}

/// Computes a fast hash of a session token.
/// 
/// A fast hash is used because session tokens are authenticated on every
/// request; using a fast hash rather than a slow password hash is OK, because
/// session tokens are randomly generated rather than chosen by the user, and
/// revoked soon enough that they cannot be feasibly brute-forced.
/// 
/// The hash is cryptographically secure, but not suitable for passwords. This
/// function should be used to hash session tokens from client cookies, in
/// order to authenticate already-logged-in users.
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
        let (raw, hash) = generate_session_token();
        
        assert_eq!(hash, token_hash(raw.as_str()), "Hash of raw token should equal the generated hash");
    }
    
    #[test]
    fn test_hash_distinct() {
        let hash1 = token_hash("example");
        let hash2 = token_hash("something else");
        
        assert_ne!(hash1, hash2, "Hashes should be distinct");
    }
}
