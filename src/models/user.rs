use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i32>,
    name: String,
    username: String,
    password: String,
    email: String,
    phone: String,
    address: String,
    payment_methods: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    acess: Option<i32>,
}

impl User {
    pub fn new(
        id: Option<i32>,
        name: String,
        username: String,
        password: String,
        email: String,
        phone: String,
        address: String,
        payment_methods: Vec<String>,
        acess: Option<i32>
    ) -> Result<User, String> {
        let mut new_user = User {
            id: id,
            name: String::new(),
            username: String::new(),
            password: String::new(),
            email: String::new(),
            phone: String::new(),
            address: String::new(),
            payment_methods: Vec::new(),
            acess: Some(0),
        };

        new_user.set_name(name)?;
        new_user.set_username(username)?;
        new_user.set_password(password)?;
        new_user.set_email(email)?;
        new_user.set_phone(phone)?;
        new_user.set_address(address)?;
        new_user.set_payment_methods(payment_methods)?;
        new_user.set_acess(acess);

        Ok(new_user)
    }

    pub fn from_json(json: web::Json<User>) -> Result<User, String> {
        Ok(User::new(
            json.id(),
            json.name(),
            json.username(),
            json.password(),
            json.email(),
            json.phone(),
            json.address(),
            json.payment_methods(),
            json.acess(),
        )?)
    }

    // Get
    pub fn id(&self) -> Option<i32> {
        self.id.clone()
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn username(&self) -> String {
        self.username.clone()
    }
    pub fn password(&self) -> String {
        self.password.clone()
    }
    pub fn email(&self) -> String {
        self.email.clone()
    }
    pub fn phone(&self) -> String {
        self.phone.clone()
    }
    pub fn address(&self) -> String {
        self.address.clone()
    }
    pub fn payment_methods(&self) -> Vec<String> {
        self.payment_methods.clone()
    }
    pub fn acess(&self) -> Option<i32> {
        self.acess.clone()
    }

    // Set
    pub fn set_name(&mut self, name: String) -> Result<(), String> {
        Self::validate_name(&name)?;
        self.name = name;
        Ok(())
    }
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
    pub fn set_email(&mut self, email: String) -> Result<(), String> {
        Self::validate_email(&email)?;
        self.email = email;
        Ok(())
    }
    pub fn set_phone(&mut self, phone: String) -> Result<(), String> {
        Self::validate_phone(&phone)?;
        self.phone = phone;
        Ok(())
    }
    pub fn set_address(&mut self, address: String) -> Result<(), String> {
        Self::validate_address(&address)?;
        self.address = address;
        Ok(())
    }
    pub fn set_payment_methods(&mut self, payment_methods: Vec<String>) -> Result<(), String> {
        Self::validate_payment_methods(&payment_methods)?;
        let mut aux = payment_methods.clone();
        aux.dedup();
        self.payment_methods = aux;
        Ok(())
    }
    pub fn set_acess(&mut self, acess: Option<i32>) {
        match acess {
            Some(_) => self.acess = acess,
            None => self.acess = Some(0),
        }
    }

    // Validate
    fn validate_name(name: &str) -> Result<(), String> {
        if name.is_empty() {
            return Err(String::from("O campo 'nome' não pode estar vazio"));
        }
        Ok(())
    }
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
    fn validate_email(email: &str) -> Result<(), String> {
        if !email.contains('@') || !email.contains('.') {
            return Err(String::from(
                "O campo 'email' deve ser um endereço de email válido",
            ));
        }
        Ok(())
    }
    fn validate_phone(phone: &str) -> Result<(), String> {
        if phone.chars().any(|c| !c.is_numeric()) {
            return Err(String::from("O campo 'phone' deve conter apenas números"));
        }
        Ok(())
    }
    fn validate_address(address: &str) -> Result<(), String> {
        if address.is_empty() {
            return Err(String::from("O campo 'address' não pode estar vazio"));
        }
        Ok(())
    }
    fn validate_payment_methods(payment_methods: &[String]) -> Result<(), String> {
        let mut aux = payment_methods.to_vec();
        aux.dedup();
        if aux.len() != payment_methods.len() {
            return Err(String::from(
                "A lista de 'payment_methods' contém elementos duplicados",
            ));
        }
        Ok(())
    }
}
