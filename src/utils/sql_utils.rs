use actix_web::web::Json;
use tokio_postgres::{NoTls, Error};
use crate::model::climbing_location::ClimbingLocation;
use std::env;
use async_trait::async_trait;

#[async_trait]
pub trait SqlUtils {
    async fn create_climbing_location(&self, location: Json<ClimbingLocation>) -> Result<i32, Error>;
}

pub struct SqlUtilsImpl;

#[async_trait]
impl SqlUtils for SqlUtilsImpl {
    async fn create_climbing_location(&self, location: Json<ClimbingLocation>) -> Result<i32, Error> {
        // Connect to the database.
        let host = env::var("SQL_CONNECTION_NAME").unwrap();
        let user = env::var("DB_USER").unwrap();
        let password = env::var("DB_PASSWORD").unwrap();
        let db_name = env::var("DB_NAME").unwrap();

        let config = format!("host={host} user={user} password={password} dbname={db_name}");

        let (client, connection) = tokio_postgres::connect(&*config, NoTls).await?;

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let query_string = format!("INSERT INTO climbing_location(name, profile_pic_location, description, address)
                                       VALUES ('{0}', '{1}', '{2}', '{3}') RETURNING id;", location.name, location.profile_pic_location, location.description, location.address);

        let row = client.query_one(&query_string, &[]).await.unwrap();
        let id: i32 = row.get("id");

        Ok(id)
    }
}
