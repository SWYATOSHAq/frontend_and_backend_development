use bcrypt::{hash, verify, BcryptError}; // DEFAULT_COST

fn hash_password(password: &str) -> Result<String, BcryptError> {
    let cost = 10;
    hash(password, cost)
}

fn verify_password(password: &str, password_hash: &str) -> Result<bool, BcryptError> {
    verify(password, password_hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing(){
    let password = "12345678";
        match hash_password(password) {
            Ok(hashed) => {
                println!("Ваш пароль захеширован: {}", hashed);
                match verify_password(password, &hashed) {
                    Ok(is_valid) => {
                        if is_valid {
                            println!("Пароль успешно проверен!");
                        } else {
                            println!("Пароль не совпадает.");
                        }
                    }
                    Err(e) => {
                        println!("Ошибка при проверке пароля: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Ошибка при хешировании пароля: {}", e);
            }
        }
    }
}
