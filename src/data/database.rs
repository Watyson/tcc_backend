use tokio_postgres::{Client, NoTls};

pub struct Db {}

impl Db {
    pub async fn connect() -> Result<Client, tokio_postgres::Error> {
        let config = "postgres://postgres:teste@localhost:5050/tcc";
        let (client, connection) = tokio_postgres::connect(&config, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(client)
    }
}

impl Drop for Db {
    fn drop(&mut self) {
        println!("Disconnecting from database...");
    }
}
