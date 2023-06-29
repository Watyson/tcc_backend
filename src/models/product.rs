use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i32>,
    name: String,
    price: f64,
    description: String,
    image: String,
    available: bool,
}

impl Product {
    pub fn new(
        id: Option<i32>,
        name: String,
        price: f64,
        description: String,
        image: String,
        available: bool,
    ) -> Result<Product, String> {
        let mut new_product = Product {
            id: id,
            name: String::new(),
            price: 0.0,
            description: String::new(),
            image: String::new(),
            available: false,
        };

        new_product.set_name(name)?;
        new_product.set_price(price)?;
        new_product.set_description(description)?;
        new_product.set_image(image)?;
        new_product.set_available(available)?;

        Ok(new_product)
    }

    pub fn from_json(json: web::Json<Product>) -> Result<Product, String> {
        Ok(Product::new(
            json.id(),
            json.name(),
            json.price(),
            json.description(),
            json.image(),
            json.available(),
        )?)
    }

    // Get
    pub fn id(&self) -> Option<i32> {
        self.id.clone()
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn price(&self) -> f64 {
        self.price.clone()
    }
    pub fn description(&self) -> String {
        self.description.clone()
    }
    pub fn image(&self) -> String {
        self.image.clone()
    }
    pub fn available(&self) -> bool {
        self.available.clone()
    }

    // Sets
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
    pub fn set_available(&mut self, available: bool) -> Result<(), String> {
        self.available = available;
        Ok(())
    }

    // Validate
    fn validate_name(name: &str) -> Result<(), String> {
        if name.is_empty() {
            return Err(String::from("Product name cannot be empty"));
        }
        Ok(())
    }
    fn validate_price(price: f64) -> Result<(), String> {
        if price <= 0.0 {
            return Err(String::from("Product price must be greater than 0"));
        }
        Ok(())
    }

    fn validate_description(description: &str) -> Result<(), String> {
        if description.is_empty() {
            return Err(String::from("Product description cannot be empty"));
        }
        Ok(())
    }

    fn validate_image(image: &str) -> Result<(), String> {
        if image.is_empty() {
            return Err(String::from("Product image cannot be empty"));
        }
        Ok(())
    }
}
