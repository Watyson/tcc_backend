use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductCartUpdate {
    status: i32,
    payment_type: String,
    id_purchase: i32,
}

impl ProductCartUpdate {
    pub fn new(status: i32, payment_type: String, id_purchase: i32) -> Result<Self, String> {
        let mut new_product_cart = Self {
            status: 0,
            payment_type: String::new(),
            id_purchase: id_purchase,
        };

        new_product_cart.set_status(status)?;
        new_product_cart.set_payment_type(payment_type)?;

        Ok(new_product_cart)
    }

    pub fn from_json(json: web::Json<ProductCartUpdate>) -> Result<ProductCartUpdate, String> {
        Ok(ProductCartUpdate::new(
            json.status(),
            json.payment_type(),
            json.id_purchase,
        )?)
    }

    // Get
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
