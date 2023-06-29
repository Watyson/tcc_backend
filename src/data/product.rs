use super::database::Db;
use crate::models::product::Product;

pub struct DbProduct {}

impl DbProduct {
    pub async fn create(product: Product) -> Result<(), String> {
        let client = Db::connect()
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        let query = "INSERT INTO tbl_product (name, price, description, image) VALUES ($1, $2, $3, $4) RETURNING id";

        client
            .execute(
                query,
                &[
                    &product.name(),
                    &product.price(),
                    &product.description(),
                    &product.image(),
                ],
            )
            .await
            .map_err(|e| format!("Failed to execute query: {}", e))?;

        Ok(())
    }

    pub async fn read(id: i32) -> Result<Product, String> {
        let client = Db::connect()
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        let query =
            "SELECT id, name, price, description, image, available FROM tbl_product WHERE id = $1";

        let row = client
            .query_one(query, &[&id])
            .await
            .map_err(|e| format!("Failed to execute query: {}", e))?;

        Ok(Product::new(
            row.get("id"),
            row.get("name"),
            row.get("price"),
            row.get("description"),
            row.get("image"),
            row.get("available"),
        )?)
    }

    pub async fn read_in_range(start: i64, limit: i64, available: bool) -> Result<Vec<Product>, String> {
        let client = Db::connect()
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        let available_condition = match available {
            true => "WHERE available = true",
            false => "",
        };
        let query = format!("SELECT id, name, price, description, image, available FROM tbl_product {} ORDER BY id LIMIT $1 OFFSET $2", available_condition);

        let rows = client
            .query(query.as_str(), &[&limit, &start])
            .await
            .map_err(|e| format!("Failed to execute query: {}", e))?;

        let mut products = Vec::new();
        for row in rows {
            let prod = Product::new(
                row.get("id"),
                row.get("name"),
                row.get("price"),
                row.get("description"),
                row.get("image"),
                row.get("available"),
            )?;

            products.push(prod);
        }

        Ok(products)
    }

    pub async fn update(id: i32, new_product: Product) -> Result<(), String> {
        let client = Db::connect()
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        let query = "UPDATE tbl_product SET name = $2, price = $3, description = $4, image = $5, available = $6 WHERE id = $1";

        client
            .execute(
                query,
                &[
                    &id,
                    &new_product.name(),
                    &new_product.price(),
                    &new_product.description(),
                    &new_product.image(),
                    &new_product.available(),
                ],
            )
            .await
            .map_err(|e| format!("Failed to execute query: {}", e))?;

        Ok(())
    }

    pub async fn delete(id: i32) -> Result<(), String> {
        let client = Db::connect()
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        let query = "DELETE FROM tbl_product WHERE id = $1";

        client
            .execute(query, &[&id])
            .await
            .map_err(|e| format!("Failed to execute query: {}", e))?;

        Ok(())
    }
}
