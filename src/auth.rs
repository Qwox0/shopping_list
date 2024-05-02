use bcrypt::verify;

fn hash(password: impl AsRef<[u8]>) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
}

// Explicitly is not Serialize/Deserialize!
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PassHash(String);

pub fn login() -> Result<(), AuthError> {
    Err(AuthError::IncorrectPassword)
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Incorrect password")]
    IncorrectPassword,
}
