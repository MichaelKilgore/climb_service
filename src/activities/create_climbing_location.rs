use::actix_web::post;
use::actix_web::HttpResponse;
use::actix_web::http::header::ContentType;
use::actix_web::web::Json;
use postgres::{Client, NoTls, Error};
use model::climbing_location;
use crate::model;
use crate::model::climbing_location::ClimbingLocation;


#[post("/create-climbing-location")]
pub async fn create_climbing_location(location: Json<ClimbingLocation>) -> HttpResponse {
    println!(
        "Received: name={}, profile_pic_location={:?}, description={}, address={}",
        location.name, location.profile_pic_location, location.description, location.address
    );
    HttpResponse::Ok().json(location.into_inner())
}
