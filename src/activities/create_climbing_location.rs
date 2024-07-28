use::actix_web::post;
use::actix_web::HttpResponse;
use::actix_web::web::Json;
use crate::model::climbing_location::ClimbingLocation;
use crate::utils::google_maps_utils::{GoogleMapsUtils, GoogleMapsUtilsImpl};
use crate::utils::sql_utils::{DbConfig, SqlUtils, SqlUtilsImpl};

#[post("/create-climbing-location")]
pub async fn create_climbing_location(location: Json<ClimbingLocation>) -> HttpResponse {
    let sql_utils = SqlUtilsImpl { db_config: DbConfig::new() };
    let google_maps_utils = GoogleMapsUtilsImpl;

    return create_climbing_location_impl(&sql_utils, &google_maps_utils, location).await;
}

async fn create_climbing_location_impl<S, T>(sql_utils: &S, google_maps_utils: &T, location: Json<ClimbingLocation>) -> HttpResponse
    where
        S: SqlUtils,
        T: GoogleMapsUtils
{
    let coords = match google_maps_utils.get_coordinates(location.address.clone()).await {
        Ok(coords) => coords,
        Err(..) => {
            return HttpResponse::InternalServerError().body("Failed to find the address provided.");
        }
    };
    
    return match sql_utils.create_climbing_location(location, coords).await {
        Ok(id) => HttpResponse::Ok().json(serde_json::json!({ "id": id })),
        Err(..) => {
            return HttpResponse::InternalServerError().body("Failed to create climbing location in sql.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::to_bytes;
    use async_trait::async_trait;
    use crate::errors::google_maps_error::GoogleMapsError;
    use crate::errors::sql_error::SqlError;
    use crate::model::coordinates::Coordinates;    

    #[actix_web::test]
    async fn test_create_climbing_location_impl_success_test() {
        let location = ClimbingLocation {
            name: "Rocky Peak".to_string(),
            profile_pic_location: "/images/rocky_peak.png".to_string(),
            description: "A popular climbing spot with diverse routes.".to_string(),
            address: "123 Climbing Lane, Boulder City".to_string(),
            additional_info: "".to_string(),
            moderator_comments: "".to_string()
        };

        pub struct SqlUtilsImplMock;

        #[async_trait]
        impl SqlUtils for SqlUtilsImplMock {
            async fn create_climbing_location(&self, _location: Json<ClimbingLocation>, _coordinates: Coordinates) -> Result<i32, SqlError> {
                Ok(4)
            }
        }

        let sql_utils = SqlUtilsImplMock;
        
        pub struct GoogleMapsUtilsMock;

        #[async_trait]
        impl GoogleMapsUtils for GoogleMapsUtilsMock {
            async fn get_coordinates(&self, _address: String) -> Result<Coordinates, GoogleMapsError> {
                Ok(Coordinates { latitude: 0.0, longitude: 0.0 })
            }
        }

        let google_maps_utils = GoogleMapsUtilsMock; 

        let resp = create_climbing_location_impl(&sql_utils, &google_maps_utils, Json(location)).await;
        
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        let resp_body = to_bytes(resp.into_body()).await.unwrap();

        let actual_json: serde_json::Value = serde_json::from_slice(&resp_body).unwrap();
        let expected_json = serde_json::json!({ "id": 4 });

        assert_eq!(expected_json, actual_json);
    }

    #[actix_web::test]
    async fn test_create_climbing_location_impl_failure_test() {
        let location = ClimbingLocation {
            name: "Rocky Peak".to_string(),
            profile_pic_location: "/images/rocky_peak.png".to_string(),
            description: "A popular climbing spot with diverse routes.".to_string(),
            address: "123 Climbing Lane, Boulder City".to_string(),
            additional_info: "".to_string(),
            moderator_comments: "".to_string()
        };

        pub struct SqlUtilsImplMock;

        #[async_trait]
        impl SqlUtils for SqlUtilsImplMock {
            async fn create_climbing_location(&self, _location: Json<ClimbingLocation>, _coordinates: Coordinates) -> Result<i32, SqlError> {
                Err(SqlError::UnknownError)
            }
        }

        let sql_utils = SqlUtilsImplMock;

        pub struct GoogleMapsUtilsMock;

        #[async_trait]
        impl GoogleMapsUtils for GoogleMapsUtilsMock {
            async fn get_coordinates(&self, _address: String) -> Result<Coordinates, GoogleMapsError> {
                Ok(Coordinates { latitude: 0.0, longitude: 0.0 })
            }
        }

        let google_maps_utils = GoogleMapsUtilsMock;

        let resp = create_climbing_location_impl(&sql_utils, &google_maps_utils, Json(location)).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

}
