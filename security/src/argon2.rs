use crate::hash::PasswordHash as PasswordHashTrait;
use argon2::{Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier};
use password_hash::{SaltString, rand_core::OsRng};

pub struct Argon2Hash<'s> {
    argon2: Argon2<'s>,
}

impl<'a> Argon2Hash<'a> {
    pub fn new(m_cost: u32, t_cost: u32, p_cost: u32, output_len: Option<usize>) -> Self {
        let params = Params::new(m_cost, t_cost, p_cost, output_len).unwrap();
        Self {
            argon2: Argon2::new(argon2::Algorithm::Argon2i, argon2::Version::V0x13, params),
        }
    }
}

impl<'a> Default for Argon2Hash<'a> {
    fn default() -> Self {
        let params = Params::new(65536, 3, 4, None).unwrap();
        Self {
            argon2: Argon2::new(argon2::Algorithm::Argon2i, argon2::Version::V0x13, params),
        }
    }
}

impl<'a> PasswordHashTrait for Argon2Hash<'a> {
    fn generate(&self, password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let pass_hash = self
            .argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap();

        pass_hash.to_string()
    }

    fn validate(&self, password: &str, hash: &str) -> bool {
        let pass_hash = PasswordHash::new(hash).unwrap();
        let resul = self.argon2.verify_password(password.as_bytes(), &pass_hash);
        resul.is_ok()
    }
}
