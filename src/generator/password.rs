use anyhow::{Context, Result};
use argon2::{password_hash::PasswordHasher, password_hash::SaltString, Argon2};

#[derive(Clone, Debug)]
pub struct PasswordGenerator {
    special_chars: Vec<char>,
}

pub struct PasswordRules {
    pub length: u32,
    pub uppercase: bool,
    pub lowercase: bool,
    pub numbers: bool,
    pub special_chars: bool,

}

impl PasswordGenerator {
    pub fn special_chars(&mut self) -> Vec<char> {
        if self.special_chars.is_empty() {
            self.special_chars = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".chars().collect();
        }

        self.special_chars.clone()
    }

    // TODO - Add rules for password generation
    pub fn generate_hash(master_pass: String, uuid: String) -> Result<String> {
        let argon2_instance = Argon2::default();
        let salt = SaltString::encode_b64(uuid.as_bytes()).expect("Failed to encode salt");
        let password_hash = argon2_instance
            .hash_password(master_pass.as_bytes(), &salt)
            .expect("Failed to hash password");

        Ok(password_hash.to_string())
    }
}
