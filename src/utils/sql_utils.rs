use actix_web::web::Json;
use tokio_postgres::{Client, NoTls};
use crate::errors::sql_error::SqlError;
use crate::model::climbing_location::ClimbLocation;
use crate::model::climb_user::ClimbUser;
use std::env;
use async_trait::async_trait;
use tokio_postgres::error::SqlState;
use crate::model::coordinates::Coordinates;

#[async_trait]
pub trait SqlUtils: Send + Sync {

    async fn create_climb_location(&self, _location: Json<ClimbLocation>, _coordinates: Coordinates) -> Result<i32, SqlError> {
        Ok(0)
    }
    async fn create_climb_user(&self, _climb_user: ClimbUser) -> Result<i32, SqlError> {
        Ok(0)
    }

    async fn update_climb_user_user_name(&self, _user_id: i32, _new_user_name: String) -> Result<(), SqlError> {
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

    async fn create_climb_location(&self, location: Json<ClimbLocation>, coordinates: Coordinates) -> Result<i32, SqlError> {
        let client = self.connect_and_spawn().await.unwrap();

        let query_string = format!("INSERT INTO climb_location(name, profile_pic_location, description, address, latitude, longitude, status, additional_info, moderator_comments)
                                       VALUES ('{0}', '{1}', '{2}', '{3}', '{4}', '{5}', '{6}', '{7}', '{8}') RETURNING id;", location.name,
                                   location.profile_pic_location, location.description, location.address, coordinates.latitude, 
                                   coordinates.longitude, "IN REVIEW", location.additional_info, "");

        let row = client.query_one(&query_string, &[]).await.unwrap();
        let id: i32 = row.get("id");

        return match client.execute(&query_string, &[]).await {
            Ok(_) => Ok(id),
            Err(..) => {
                return Err(SqlError::UnknownError);
            }
        }
    }

    async fn create_climb_user(&self, climb_user: ClimbUser) -> Result<i32, SqlError> {
        let client = self.connect_and_spawn().await.unwrap();

        let insert_string = format!("INSERT INTO climb_user(user_name, status, moderator_comments)
                                       VALUES ('{0}', '{1}', '{2}') RETURNING id;", climb_user.user_name, climb_user.status, climb_user.moderator_comments);

        return match client.query_one(&insert_string, &[]).await {
            Ok(row) => Ok(row.get("id")),
            Err(err) => {
                if err.code() == Some(&SqlState::UNIQUE_VIOLATION) {
                    return Err(SqlError::PrimaryKeyAlreadyExists);
                }
                return Err(SqlError::UnknownError);
            }
        }
    }

    async fn update_climb_user_user_name(&self, user_id: i32, new_user_name: String) -> Result<(), SqlError> {
        let client = self.connect_and_spawn().await.unwrap();

        let query = format!("UPDATE climb_user
                                    SET user_name = '{0}'
                                    WHERE id = '{1}'", new_user_name, user_id);
        
        return match client.execute(&query, &[]).await {
            Ok(_) => Ok(()),
            Err(err) => {
                if err.code() == Some(&SqlState::UNIQUE_VIOLATION) {
                    return Err(SqlError::PrimaryKeyAlreadyExists);
                }
                return Err(SqlError::UnknownError);
            }
        }
    }
}

impl SqlUtilsImpl {
   async fn connect_and_spawn(&self) -> Result<Client, SqlError> {
       let config = self.db_config.get_config_string();

       let (client, connection) = match tokio_postgres::connect(&*config, NoTls).await {
           Ok((client, connection)) => (client, connection),
           Err(err) => return Err(SqlError::ConnectionError(err.to_string())),
       };

       // The connection object performs the actual communication with the database,
       // so spawn it off to run on its own.
       tokio::spawn(async move {
           if let Err(e) = connection.await {
               eprintln!("connection error: {}", e);
           }
       });

       return Ok(client);
   }
}