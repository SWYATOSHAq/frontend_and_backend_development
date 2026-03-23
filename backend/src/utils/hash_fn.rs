use bcrypt::{hash, verify, BcryptError};

pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    let cost = 10;
    hash(password, cost)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, BcryptError> {
    verify(password, password_hash)
}