use::actix_web::post;
use::actix_web::HttpResponse;
use::actix_web::web::Json;
use crate::model::climbing_location::ClimbingLocation;
use crate::sql_utils;

#[post("/create-climbing-location")]
pub async fn create_climbing_location(location: Json<ClimbingLocation>) -> HttpResponse {
    return match sql_utils::create_climbing_location::create_climbing_location(location).await {
        Ok(()) => HttpResponse::Ok().json("{}"),
        Err(err) => {
            println!("Error: {}", err);
            return HttpResponse::InternalServerError().body("Failed to create climbing location in sql.");
        }
    }
}
