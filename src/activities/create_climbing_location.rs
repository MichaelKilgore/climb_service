use::actix_web::post;
use::actix_web::HttpResponse;
use::actix_web::web::Json;
use crate::model::climbing_location::ClimbingLocation;
use crate::utils::sql_utils::{SqlUtils, SqlUtilsImpl};

#[post("/create-climbing-location")]
pub async fn create_climbing_location(location: Json<ClimbingLocation>) -> HttpResponse {
    let sql_utils = SqlUtilsImpl;

    return create_climbing_location_impl(&sql_utils, location).await;
}

async fn create_climbing_location_impl<S>(sql_utils: &S, location: Json<ClimbingLocation>) -> HttpResponse
    where
        S: SqlUtils
{
    return match sql_utils.create_climbing_location(location).await {
        Ok(id) => HttpResponse::Ok().json(serde_json::json!({ "id": id })),
        Err(err) => {
            println!("Error: {}", err);
            return HttpResponse::InternalServerError().body("Failed to create climbing location in sql.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::to_bytes;
    use async_trait::async_trait;
    use tokio_postgres::Error;

    #[actix_web::test]
    async fn test_create_climbing_location_impl_success_test() {
        let location = ClimbingLocation {
            name: "Rocky Peak".to_string(),
            profile_pic_location: "/images/rocky_peak.png".to_string(),
            description: "A popular climbing spot with diverse routes.".to_string(),
            address: "123 Climbing Lane, Boulder City".to_string(),
            additional_info: "".to_string()
        };

        pub struct SqlUtilsImplMock;

        #[async_trait]
        impl SqlUtils for SqlUtilsImplMock {
            async fn create_climbing_location(&self, _location: Json<ClimbingLocation>) -> Result<i32, Error> {
                Ok(4)
            }
        }

        let sql_utils = SqlUtilsImplMock;

        let resp = create_climbing_location_impl(&sql_utils, Json(location)).await;
        
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        let resp_body = to_bytes(resp.into_body()).await.unwrap();

        let actual_json: serde_json::Value = serde_json::from_slice(&resp_body).unwrap();
        let expected_json = serde_json::json!({ "id": 4 });

        assert_eq!(expected_json, actual_json);
    }

}
