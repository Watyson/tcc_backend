use super::database::Db;
use crate::models::product_cart::ProductCart;

pub struct DbHistoric {}

impl DbHistoric {
    pub async fn create(items: &[ProductCart], user_id: i32) -> Result<(), String> {
        let client = Db::connect()
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        let query = "INSERT INTO tbl_cart_item (name, observation, quantity, price, description, image, idtbl_user, status, date, payment_type, id_purchase)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)";
        
        for item in items {

            client
                .execute(
                    query,
                    &[
                        &item.name(),
                        &item.observation(),
                        &item.quantity(),
                        &item.price(),
                        &item.description(),
                        &item.image(),
                        &user_id,
                        &item.status(),
                        &item.date(),
                        &item.payment_type(),
                        &item.id_purchase(),
                    ],
                )
                .await
                .map_err(|e| format!("Failed to execute query: {}", e))?;
        }

        Ok(())
    }

    pub async fn read(user_id: i32) -> Result<Vec<ProductCart>, String> {
        let client = Db::connect()
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        let query = "SELECT id, name, price, description, image, date, quantity, observation, status, payment_type, id_purchase FROM tbl_cart_item WHERE idtbl_user = $1 ORDER BY date DESC";
        let mut items = Vec::new();

        let rows = client
            .query(query, &[&user_id])
            .await
            .map_err(|e| format!("Failed to execute query: {}", e))?;

        for row in rows {
            let item = ProductCart::new(
                Some(row.get(0)),
                row.get(1),
                row.get(2),
                row.get(3),
                row.get(4),
                row.get(5),
                row.get(6),
                row.get(7),
                row.get(8),
                row.get(9),
                row.get(10),
            )
            .map_err(|e| format!("Failed to create ProductCart: {}", e))?;

            items.push(item);
        }

        Ok(items)
    }

    pub async fn update(id_purchase: i32, new_status: i32, new_payment_type: String) -> Result<(), String> {
        let client = Db::connect()
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;
    
        let query = "UPDATE tbl_cart_item SET status = $1, payment_type = $2 WHERE id_purchase = $3";
    
        client
            .execute(
                query,
                &[
                    &new_status,
                    &new_payment_type,
                    &id_purchase,
                ],
            )
            .await
            .map_err(|e| format!("Failed to execute query: {}", e))?;
    
        Ok(())
    }

    /*
    pub async fn delete(id_purchase: i32) -> Result<(), String> {
        let client = Db::connect()
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;
    
        let query = "DELETE FROM tbl_cart_item WHERE id_purchase = $1";
    
        client
            .execute(query, &[&id_purchase])
            .await
            .map_err(|e| format!("Failed to execute query: {}", e))?;
    
        Ok(())
    }
    */

    pub async fn read_all() -> Result<Vec<ProductCart>, String> {
        let client = Db::connect()
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        let query = "SELECT id, name, price, description, image, date, quantity, observation, status, payment_type, id_purchase FROM tbl_cart_item ORDER BY date ASC";
        let mut items = Vec::new();

        let rows = client
            .query(query, &[])
            .await
            .map_err(|e| format!("Failed to execute query: {}", e))?;

        for row in rows {
            let item = ProductCart::new(
                Some(row.get(0)),
                row.get(1),
                row.get(2),
                row.get(3),
                row.get(4),
                row.get(5),
                row.get(6),
                row.get(7),
                row.get(8),
                row.get(9),
                row.get(10),
            )
            .map_err(|e| format!("Failed to create ProductCart: {}", e))?;

            items.push(item);
        }

        Ok(items)
    }
}
