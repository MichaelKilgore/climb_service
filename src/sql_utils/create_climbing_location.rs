use actix_web::web::Json;
use tokio_postgres::{NoTls, Error, SimpleQueryMessage};
use crate::model::climbing_location::ClimbingLocation;
use std::env;

pub async fn create_climbing_location(location: Json<ClimbingLocation>) -> Result<(), Error> {
    // Connect to the database.
    let host = env::var("SQL_CONNECTION_NAME").unwrap();
    let user = env::var("DB_USER").unwrap();
    let password = env::var("DB_PASSWORD").unwrap();
    let db_name = env::var("DB_NAME").unwrap();

    let config = format!("host=/cloudsql/{host} user={user} password={password} dbname={db_name}");

    let (client, connection) = tokio_postgres::connect(&*config, NoTls).await?;

    eprintln!("ONE");

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    eprintln!("TWO");

    // Now we can execute a simple statement that just returns its parameter.
    let messages = client
        .simple_query(
            "CREATE TEMPORARY TABLE foo (
                id SERIAL,
                name TEXT
            );
            INSERT INTO foo (name) VALUES ('hello'), ('world');
            SELECT * FROM climbing_location ORDER BY id",
        )
        .await
        .unwrap();

    eprintln!("THREE");

    let mut word = "";

    match messages[0] {
        SimpleQueryMessage::CommandComplete(0) => {}
        _ => panic!("unexpected message"),
    }
    match messages[1] {
        SimpleQueryMessage::CommandComplete(2) => {}
        _ => panic!("unexpected message"),
    }
    match &messages[2] {
        SimpleQueryMessage::Row(row) => {
            println!("{}", row.get(1).unwrap());
            word = row.get(1).unwrap();
        }
        _ => panic!("unexpected message"),
    }

    println!(
        "Received: word={}",
        word
    );

    Ok(())
}
