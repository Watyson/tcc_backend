use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

impl Credentials {
    pub fn new(username: String, password: String) -> Result<Self, String> {
        let mut credentials = Self {
            username: String::new(),
            password: String::new(),
        };
        credentials.set_username(username)?;
        credentials.set_password(password)?;

        Ok(credentials)
    }

    pub fn from_json(json: web::Json<Credentials>) -> Result<Credentials, String> {
        let credentials = Credentials::new(json.username(), json.password())?;
        Ok(credentials)
    }

    // Get
    pub fn username(&self) -> String {
        self.username.clone()
    }
    pub fn password(&self) -> String {
        self.password.clone()
    }

    //Set
    pub fn set_username(&mut self, username: String) -> Result<(), String> {
        Self::validate_username(&username)?;
        self.username = username;
        Ok(())
    }
    pub fn set_password(&mut self, password: String) -> Result<(), String> {
        Self::validate_password(&password)?;
        self.password = password;
        Ok(())
    }

    // Validate
    fn validate_username(username: &str) -> Result<(), String> {
        if username.len() > 50 {
            return Err(String::from(
                "O campo 'username' deve ter no máximo 50 caracteres",
            ));
        }
        Ok(())
    }
    fn validate_password(password: &str) -> Result<(), String> {
        if password.len() < 4 {
            return Err(String::from(
                "O campo 'password' deve ter pelo menos 8 caracteres",
            ));
        }
        Ok(())
    }
}
