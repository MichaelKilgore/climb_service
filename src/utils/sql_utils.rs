use actix_web::web::Json;
use tokio_postgres::{NoTls, Error};
use crate::model::climbing_location::ClimbingLocation;
use crate::model::climb_user::ClimbUser;
use std::env;
use async_trait::async_trait;

#[async_trait]
pub trait SqlUtils: Send + Sync {

    async fn create_climbing_location(&self, _location: Json<ClimbingLocation>) -> Result<i32, Error> {
        Ok(0)
    }
    async fn create_climb_user(&self, _climb_user: ClimbUser) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Clone)]
pub struct DbConfig {
    host: String,
    user: String,
    password: String,
    db_name: String,
}

impl DbConfig {
    pub(crate) fn new() -> Self {
        DbConfig {
            host: env::var("SQL_CONNECTION_NAME").unwrap(),
            user: env::var("DB_USER").unwrap(),
            password: env::var("DB_PASSWORD").unwrap(),
            db_name: env::var("DB_NAME").unwrap(),
        }
    }

    fn get_config_string(&self) -> String {
        format!(
            "host={} user={} password={} dbname={}",
            self.host, self.user, self.password, self.db_name
        )
    }
}

pub struct SqlUtilsImpl { 
    pub(crate) db_config: DbConfig
}

#[async_trait]
impl SqlUtils for SqlUtilsImpl {
    
    async fn create_climbing_location(&self, location: Json<ClimbingLocation>) -> Result<i32, Error> {
        let config = self.db_config.get_config_string();

        let (client, connection) = tokio_postgres::connect(&*config, NoTls).await?;

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let query_string = format!("INSERT INTO climbing_location(name, profile_pic_location, description, address, status, additional_info, moderator_comments)
                                       VALUES ('{0}', '{1}', '{2}', '{3}', '{4}', '{5}', '{6}') RETURNING id;", location.name,
                                   location.profile_pic_location, location.description, location.address, "IN REVIEW", location.additional_info, "");

        let row = client.query_one(&query_string, &[]).await.unwrap();
        let id: i32 = row.get("id");

        Ok(id)
    }

    async fn create_climb_user(&self, climb_user: ClimbUser) -> Result<(), Error> {
        let config = self.db_config.get_config_string();

        let (client, connection) = tokio_postgres::connect(&*config, NoTls).await?;

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let query_string = format!("INSERT INTO climb_user(user_name, phone_number, status, moderator_comments)
                                       VALUES ('{0}', '{1}', '{2}', '{3}');", climb_user.user_name, climb_user.phone_number, climb_user.status, climb_user.moderator_comments);
        
        let _ = client.query_one(&query_string, &[]).await;

        Ok(())
    }
}
