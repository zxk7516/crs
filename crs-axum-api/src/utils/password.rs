use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

use crate::errors::{OError, OResult};

#[derive(Clone)]
pub struct PasswordUtil<'key> {
    argon2: Argon2<'key>,
    salt: SaltString,
}

impl<'key> PasswordUtil<'key> {
    pub fn new(key: &'key [u8]) -> Self {
        let params = argon2::Params::default();
        Self {
            argon2: Argon2::new(
                Some(key), //None,
                params.t_cost,
                params.m_cost,
                params.p_cost,
                params.version,
            )
            .unwrap(),
            salt: SaltString::generate(&mut rand_core::OsRng),
        }
    }

    pub fn hash_password(&self, password: &str) -> OResult<String> {
        self.argon2
            .hash_password_simple(password.as_bytes(), self.salt.as_ref())
            .map_err(|_e| OError::InternalError("hash error"))
            .map(|s| s.to_string())
    }

    pub fn verify_password(&self, hashed_password: &str, input_password: &str) -> bool {
        let parsed_hash = PasswordHash::new(&hashed_password).unwrap();
        self.argon2
            .verify_password(&input_password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}

impl<'key> Default for PasswordUtil<'key> {
    fn default() -> Self {
        Self {
            argon2: Argon2::default(),
            salt: SaltString::generate(&mut rand_core::OsRng),
        }
    }
}
