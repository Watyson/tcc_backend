use serde::{Serialize};

#[derive(Serialize)]
pub struct LoginResponse {
    id: i32,
    token: String,
}

impl LoginResponse {
    pub fn new(id: i32, token: String) -> LoginResponse {
        LoginResponse {
            id: id,
            token: token,
        }
    }
}