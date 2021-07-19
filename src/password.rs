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
        println!("{:?}, {:?}", hashed_password, input_password);
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_password_hash() {
        let plain_password = "abc";
        let auth = AuthenticateUtils::new();
        let hash_password = auth.hash_password(plain_password).unwrap();
        println!("{}", hash_password);
        let result = auth.verify_password(&hash_password, plain_password);
        println!("{}", result);
    }
}
