use crate::data::database::Db;
use crate::models::user::User;

pub struct DbUser {}

impl DbUser {
    pub async fn create(new_user: User) -> Result<(), String> {
        if Self::username_exists(&new_user.username()).await? {
            return Err("Username already exists.".to_string());
        }

        let client = Db::connect()
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        let query = "INSERT INTO tbl_user (name, username, password, email, phone, address, acess) VALUES ($1, $2, $3, $4, $5, $6, $7)";

        client
            .execute(
                query,
                &[
                    &new_user.name(),
                    &new_user.username(),
                    &new_user.password(),
                    &new_user.email(),
                    &new_user.phone(),
                    &new_user.address(),
                    &new_user.acess(),
                ],
            )
            .await
            .map_err(|e| format!("Failed to execute query: {}", e))?;

        Ok(())
    }

    pub async fn read(id: i32) -> Result<User, String> {
        let client = Db::connect()
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        let query = "SELECT id, name, username, password, email, phone, address, acess FROM tbl_user WHERE id = $1";

        let row = client
            .query_one(query, &[&id])
            .await
            .map_err(|e| format!("Failed to execute query: {}", e))?;

        Ok(User::new(
            row.get("id"),
            row.get("name"),
            row.get("username"),
            row.get("password"),
            row.get("email"),
            row.get("phone"),
            row.get("address"),
            Vec::new(),
            row.get("acess"),
        )?)
    }

    pub async fn update(new_user: User) -> Result<(), String> {
        let client = Db::connect()
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        let query = "UPDATE tbl_user SET name = $1, password = $3, email = $4, phone = $5, address = $6, acess = $7 WHERE username = $2";

        let rows_affected = client
            .execute(
                query,
                &[
                    &new_user.name(),
                    &new_user.username(),
                    &new_user.password(),
                    &new_user.email(),
                    &new_user.phone(),
                    &new_user.address(),
                    &new_user.acess(),
                ],
            )
            .await
            .map_err(|e| format!("Failed to execute query: {}", e))?;

        if rows_affected != 1 {
            return Err("Failed to update, incorrect data.".to_string());
        }

        Ok(())
    }

    pub async fn update_password(email: &str, new_password: &str) -> Result<(), String> {
        let client = Db::connect()
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        let query = "UPDATE tbl_user SET password = $1 WHERE email = $2";

        let rows_affected = client
            .execute(query, &[&new_password, &email])
            .await
            .map_err(|e| format!("Failed to execute query: {}", e))?;

        if rows_affected != 1 {
            return Err("Falha ao atualizar a senha, e-mail ou dados incorretos.".to_string());
        }

        Ok(())
    }

    pub async fn delete(id: i32) -> Result<(), String> {
        let client = Db::connect()
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        let query = "DELETE FROM tbl_user WHERE id = $1";

        client
            .execute(query, &[&id])
            .await
            .map_err(|e| format!("Failed to execute query: {}", e))?;

        Ok(())
    }

    pub async fn username_exists(username: &str) -> Result<bool, String> {
        let client = Db::connect()
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        let query = "SELECT EXISTS (SELECT 1 FROM tbl_user WHERE username = $1)";

        let exists: bool = client
            .query_one(query, &[&username])
            .await
            .map_err(|e| format!("Failed to execute query: {}", e))?
            .get(0);

        Ok(exists)
    }

    pub async fn autenticate(username: String, password: String) -> Result<i32, String> {
        let client = Db::connect()
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        let query = "SELECT id FROM tbl_user WHERE username = $1 AND password = $2";

        match client
            .query_opt(query, &[&username, &password])
            .await
            .map_err(|e| format!("Failed to execute query: {}", e))?
        {
            Some(row) => Ok(row.get("id")),
            None => Err("Invalid username or password".to_string()),
        }
    }

    pub async fn read_acess(id: i32) -> Result<i32, String> {
        let client = Db::connect()
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        let query = "SELECT acess FROM tbl_user WHERE id = $1";

        let row = client
            .query_one(query, &[&id])
            .await
            .map_err(|e| format!("Failed to execute query: {}", e))?;

        Ok(row.get("acess"))
    }

    pub async fn read_admins() -> Result<Vec<User>, String> {
        let client = Db::connect()
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        let query = "SELECT id, name, username, password, email, phone, address, acess FROM tbl_user WHERE acess = 1 OR acess = 2";
        let mut users = Vec::new();

        for row in client
            .query(query, &[])
            .await
            .map_err(|e| format!("Failed to execute query: {}", e))?
        {
            let user = User::new(
                row.get("id"),
                row.get("name"),
                row.get("username"),
                row.get("password"),
                row.get("email"),
                row.get("phone"),
                row.get("address"),
                Vec::new(),
                row.get("acess"),
            )?;

            users.push(user);
        }

        Ok(users)
    }
}
