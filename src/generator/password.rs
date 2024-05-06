use anyhow::{Context, Result};
use argon2::{password_hash::PasswordHasher, password_hash::SaltString, Argon2};

#[derive(Clone, Debug)]
pub struct PasswordGenerator {
    special_chars: Vec<char>,
}

pub struct PasswordRuleLength {
    pub min: u8,
    pub max: u8,
}

pub struct PasswordAllowed {
    pub letters: bool,
    pub numbers: bool,
    pub special_chars: bool,
    pub uppercase: bool,
    pub lowercase: bool,
}

pub struct PasswordRules {
    pub length: Option<PasswordRuleLength>,
    pub letters: Option<PasswordRuleLength>,
    pub numbers: Option<PasswordRuleLength>,
    pub special_chars: Option<PasswordRuleLength>,
    pub uppercase: Option<PasswordRuleLength>,
    pub lowercase: Option<PasswordRuleLength>,
    pub allowed: PasswordAllowed,
    pub version: Option<u8>,
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
