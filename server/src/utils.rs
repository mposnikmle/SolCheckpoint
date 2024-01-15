use argon2::{self, Config};
use rand::Rng;

fn hash_password(password: &str) -> Result<String, argon2::Error> {
    // Generate a random salt
    let salt: [u8; 32] = rand::thread_rng().gen();

    // Configure Argon2
    let config = Config::default();
    
    // Hash the password with Argon2
    let hash = argon2::hash_encoded(password.as_bytes(), &salt, &config)?;

    Ok(hash)
}