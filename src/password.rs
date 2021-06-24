use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

use crate::errors::{MyError, MyResult};

#[derive(Clone)]
pub struct AuthenticateUtils<'key> {
    argon2: Argon2<'key>,
    salt: SaltString,
}

impl<'key> AuthenticateUtils<'key> {
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
            salt: SaltString::generate(&mut rand_core::OsRng),
        }
    }

    pub fn hash_password(&self, password: &str) -> MyResult<String> {
        self.argon2
            .hash_password_simple(password.as_bytes(), self.salt.as_ref())
            .map_err(|_e| MyError::InternalError)
            .map(|s| s.to_string())
    }

    pub fn verify_password(&self, hashed_password: &str, input_password: &str) -> bool {
        let parsed_hash = PasswordHash::new(&hashed_password).unwrap();
        self.argon2
            .verify_password(&input_password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}

impl<'key> Default for AuthenticateUtils<'key> {
    fn default() -> Self {
        Self::new()
    }
}
