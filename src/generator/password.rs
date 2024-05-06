use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};

pub fn generate_password(master_pass: String, uuid: String) -> String {
    let argon2_instance = Argon2::default();
    let salt = SaltString::encode_b64(uuid.as_bytes()).unwrap();
    let password_hash = argon2_instance
        .hash_password(master_pass.as_bytes(), &salt)
        .unwrap();

    return password_hash.to_string();
}
