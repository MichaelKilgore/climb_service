use::actix_web::{get, HttpResponse, http::header::ContentType};

#[get("/hello")]
pub async fn hello() -> HttpResponse {

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json("{\"message\": \"hello Michael!\"}")
}
