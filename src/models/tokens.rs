use lazy_static::lazy_static;
use rand;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

lazy_static! {
    static ref TOKENS: Mutex<HashMap<String, (i32, SystemTime)>> = Mutex::new(HashMap::new());
}

pub fn generate_token(user_id: i32) -> Result<String, String> {
    if let Some(token) = find_token_by_id(user_id) {
        renew_token(&token, user_id)?;
        return Ok(token);
    }
    let token = format!("{:x}", rand::random::<u128>());
    let expiration = SystemTime::now() + Duration::from_secs(3600); // expira em 1 hora
    TOKENS
        .lock()
        .unwrap()
        .insert(token.clone(), (user_id, expiration));
    Ok(token)
}

pub fn find_token_by_id(user_id: i32) -> Option<String> {
    let tokens = TOKENS.lock().unwrap();
    for (token, (id, expiration)) in tokens.iter() {
        if *id == user_id && *expiration > SystemTime::now() {
            return Some(token.clone());
        }
    }
    None
}

pub fn find_user_id_by_token(token: &str) -> Option<i32> {
    let mut tokens = TOKENS.lock().unwrap();
    if let Some((user_id, expiration)) = tokens.get(token) {
        if *expiration > SystemTime::now() {
            return Some(user_id.clone());
        } else {
            tokens.remove(token);
        }
    }
    None
}

pub fn renew_token(token: &str, user_id: i32) -> Result<(), String> {
    let mut tokens = TOKENS
        .lock()
        .map_err(|e| format!("Failed to lock tokens: {}", e))?;

    if let Some((id, expiration)) = tokens.get(token) {
        if *id == user_id && *expiration > SystemTime::now() {
            let new_expiration = SystemTime::now() + Duration::from_secs(3600); // expira em 1 hora
            tokens.insert(token.to_string(), (user_id, new_expiration));
            return Ok(());
        }
    }
    Err("Token not found or expired. Please login again.".to_string())
}

pub fn remove_token(token: &str) {
    TOKENS.lock().unwrap().remove(token);
}

pub fn expire_tokens() {
    let mut tokens = TOKENS.lock().unwrap();
    let expired_tokens: Vec<String> = tokens
        .iter()
        .filter(|(_, (_, expiration))| *expiration <= SystemTime::now())
        .map(|(token, _)| token.clone())
        .collect();
    for token in expired_tokens {
        tokens.remove(&token);
    }
}
