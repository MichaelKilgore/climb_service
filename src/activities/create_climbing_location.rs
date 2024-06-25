use futures::FutureExt;
use::actix_web::post;
use::actix_web::HttpResponse;
use::actix_web::http::header::ContentType;
use::actix_web::web::Json;
use tokio::net::TcpStream;
use model::climbing_location;
use crate::model;
use crate::model::climbing_location::ClimbingLocation;
use tokio_postgres::{Client, Config, Connection, Error, SimpleQueryMessage};
use tokio_postgres::tls::{NoTls, NoTlsStream};



#[post("/create-climbing-location")]
pub async fn create_climbing_location(mut location: Json<ClimbingLocation>) -> HttpResponse {
    // let mut client = Client::connect(&"34.30.64.62/postgres", NoTls);
    // postgres://postgres:Montero17$@34.30.64.62:5432/postgres
    // postgres://myuser:mypassword@34.123.45.67/mydatabase
    // "host=34.30.64.62 user=postgres"
    // "host=localhost user=postgres password=Montero17$"
    eprintln!("ASFAF");
    let client = connect("host=34.30.64.62 user=postgres password=postgres sslmode=disable dbname=postgres").await;
    eprintln!("HELLO WORLD");

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
        "Received: name={}, profile_pic_location={:?}, description={}, address={}",
        location.name, location.profile_pic_location, location.description, word
    );
    location.address = word.parse().unwrap();
    HttpResponse::Ok().json(location.into_inner())
}


async fn connect_raw(s: &str) -> Result<(Client, Connection<TcpStream, NoTlsStream>), Error> {
    let socket = TcpStream::connect("127.0.0.1:5432").await.unwrap();
    let config = s.parse::<Config>().unwrap();
    config.connect_raw(socket, NoTls).await
}

async fn connect(s: &str) -> Client {
    let (client, connection) = connect_raw(s).await.unwrap();
    let connection = connection.map(|r| r.unwrap());
    tokio::spawn(connection);
    client
}
