use crate::Result;

pub fn hash_pwd(password: &str) -> Result<String> {
    let p = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
    Ok(p)
}

pub fn verify_pwd(password: &str, hashed_password: &str) -> Result<bool> {
    let r = bcrypt::verify(password, hashed_password)?;
    Ok(r)
}

pub fn new_id() -> String {
    xid::new().to_string()
}
