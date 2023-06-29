use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductCart {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i32>,
    name: String,
    price: f64,
    description: String,
    image: String,
    date: String,
    quantity: i32,
    observation: String,
    status: i32,
    payment_type: String,
    id_purchase: i32,
}

impl ProductCart {
    pub fn new(
        id: Option<i32>,
        name: String,
        price: f64,
        description: String,
        image: String,
        date: String,
        quantity: i32,
        observation: String,
        status: i32,
        payment_type: String,
        id_purchase: i32,
    ) -> Result<Self, String> {
        let mut new_product_cart = Self {
            id: id,
            name: String::new(),
            price: 0.0,
            description: String::new(),
            image: String::new(),
            date: String::new(),
            quantity: 0,
            observation: String::new(),
            status: 0,
            payment_type: String::new(),
            id_purchase: id_purchase,
        };

        new_product_cart.set_name(name)?;
        new_product_cart.set_price(price)?;
        new_product_cart.set_description(description)?;
        new_product_cart.set_image(image)?;
        new_product_cart.set_date(date)?;
        new_product_cart.set_quantity(quantity)?;
        new_product_cart.set_observation(observation)?;
        new_product_cart.set_status(status)?;
        new_product_cart.set_payment_type(payment_type)?;

        Ok(new_product_cart)
    }

    pub fn from_json(json: web::Json<Vec<ProductCart>>) -> Result<Vec<ProductCart>, String> {
        let mut product_carts = Vec::new();
        for product_cart in json.iter() {
            let prod = ProductCart::new(
                product_cart.id(),
                product_cart.name(),
                product_cart.price(),
                product_cart.description(),
                product_cart.image(),
                product_cart.date(),
                product_cart.quantity(),
                product_cart.observation(),
                product_cart.status(),
                product_cart.payment_type(),
                product_cart.id_purchase(),
            )?;
            product_carts.push(prod);
        }
        Ok(product_carts)
    }

    // Get
    pub fn id(&self) -> Option<i32> {
        self.id
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn price(&self) -> f64 {
        self.price
    }
    pub fn description(&self) -> String {
        self.description.clone()
    }
    pub fn image(&self) -> String {
        self.image.clone()
    }
    pub fn date(&self) -> String {
        self.date.clone()
    }
    pub fn quantity(&self) -> i32 {
        self.quantity
    }
    pub fn observation(&self) -> String {
        self.observation.clone()
    }
    pub fn status(&self) -> i32 {
        self.status.clone()
    }
    pub fn payment_type(&self) -> String {
        self.payment_type.clone()
    }
    pub fn id_purchase(&self) -> i32 {
        self.id_purchase.clone()
    }

    // Set
    pub fn set_name(&mut self, name: String) -> Result<(), String> {
        Self::validate_name(&name)?;
        self.name = name;
        Ok(())
    }
    pub fn set_price(&mut self, price: f64) -> Result<(), String> {
        Self::validate_price(price)?;
        self.price = price;
        Ok(())
    }
    pub fn set_description(&mut self, description: String) -> Result<(), String> {
        Self::validate_description(&description)?;
        self.description = description;
        Ok(())
    }
    pub fn set_image(&mut self, image: String) -> Result<(), String> {
        Self::validate_image(&image)?;
        self.image = image;
        Ok(())
    }
    pub fn set_date(&mut self, date: String) -> Result<(), String> {
        Self::validate_date(&date)?;
        self.date = date;
        Ok(())
    }
    pub fn set_quantity(&mut self, quantity: i32) -> Result<(), String> {
        Self::validate_quantity(quantity)?;
        self.quantity = quantity;
        Ok(())
    }
    pub fn set_observation(&mut self, observation: String) -> Result<(), String> {
        Self::validate_observation(&observation)?;
        self.observation = observation;
        Ok(())
    }
    pub fn set_status(&mut self, status: i32) -> Result<(), String> {
        Self::validate_status(status)?;
        self.status = status;
        Ok(())
    }
    pub fn set_payment_type(&mut self, payment_type: String) -> Result<(), String> {
        Self::validate_payment_type(&payment_type)?;
        self.payment_type = payment_type;
        Ok(())
    }

    // Validate
    fn validate_name(name: &str) -> Result<(), String> {
        if name.is_empty() {
            return Err(String::from("Name cannot be empty"));
        }
        if name.len() > 255 {
            return Err(String::from("Name is too long"));
        }
        Ok(())
    }
    fn validate_price(price: f64) -> Result<(), String> {
        if price <= 0.0 {
            return Err(String::from("Price must be greater than zero"));
        }
        Ok(())
    }
    fn validate_description(description: &str) -> Result<(), String> {
        if description.is_empty() {
            return Err(String::from("Description cannot be empty"));
        }
        Ok(())
    }
    fn validate_image(image: &str) -> Result<(), String> {
        if image.is_empty() {
            return Err(String::from("Image cannot be empty"));
        }
        Ok(())
    }
    fn validate_date(date: &str) -> Result<(), String> {
        if date.is_empty() {
            return Err(String::from("Date cannot be empty"));
        }
        Ok(())
    }
    fn validate_quantity(quantity: i32) -> Result<(), String> {
        if quantity <= 0 {
            return Err(String::from("Quantity must be greater than zero"));
        }
        Ok(())
    }
    fn validate_observation(observation: &str) -> Result<(), String> {
        if observation.len() > 255 {
            return Err(String::from("Observation is too long"));
        }
        Ok(())
    }
    fn validate_status(status: i32) -> Result<(), String> {
        if status != 0 && status != 1 && status != 2 && status != 3 && status != 4 {
            return Err(String::from(
                "Status must be '0' or '1' or '2' or '3' or '4'",
            ));
        }
        Ok(())
    }

    fn validate_payment_type(payment_type: &str) -> Result<(), String> {
        if payment_type.is_empty() {
            return Err(String::from("Payment tpe cannot be empty"));
        }
        Ok(())
    }
}
