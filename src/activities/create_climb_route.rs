use::actix_web::post;
use::actix_web::HttpResponse;
use::actix_web::web::Json;
use crate::model::climb_route::ClimbRoute;
use crate::utils::sql_utils::{DbConfig, SqlUtils, SqlUtilsImpl};


#[post("/create-climb-route")]
pub async fn create_climb_route(route: Json<ClimbRoute>) -> HttpResponse {
    let sql_utils = SqlUtilsImpl { db_config: DbConfig::new() };

    return create_climb_route_impl(&sql_utils, route).await;
}

async fn create_climb_route_impl<S>(sql_utils: &S, route: Json<ClimbRoute>) -> HttpResponse
    where
        S: SqlUtils
{
    return match sql_utils.create_climb_route(route).await {
        Ok(id) => HttpResponse::Created().json(serde_json::json!({ "climb_route_id": id })),
        Err(..) => {
            return HttpResponse::InternalServerError().body("Failed to create climb route in sql.");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::to_bytes;
    use async_trait::async_trait;
    use crate::errors::sql_error::SqlError;

    #[actix_web::test]
    async fn test_create_climb_route_impl_success_test(){
        let climb_route = ClimbRoute {
            name: "The Warmup Problem".to_string(),
            grade: "V3".to_string(),
            climb_location_id: 1,
            latitude: 55.0,
            longitude: -31.65,
            description: "".to_string(),
            video_link: "/videolink.com".to_string(),
            climb_user_id: 1
        };

        pub struct SqlUtilsImplMock;

        #[async_trait]
        impl SqlUtils for SqlUtilsImplMock {
            async fn create_climb_route(&self, _climb_route: Json<ClimbRoute>) -> Result<i32, SqlError> {
                Ok(4)
            }
        }

        let sql_utils = SqlUtilsImplMock;

        let resp = create_climb_route_impl(&sql_utils, Json(climb_route)).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);

        let resp_body = to_bytes(resp.into_body()).await.unwrap();

        let actual_json: serde_json::Value = serde_json::from_slice(&resp_body).unwrap();
        let expected_json = serde_json::json!({ "climb_route_id": 4 });

        assert_eq!(expected_json, actual_json);

    }

    #[actix_web::test]
    async fn test_create_climb_route_impl_failure_test() {
        let climb_route = ClimbRoute {
            name: "The Warmup Problem".to_string(),
            grade: "V3".to_string(),
            climb_location_id: 1,
            latitude: 55.0,
            longitude: -31.65,
            description: "".to_string(),
            video_link: "/videolink.com".to_string(),
            climb_user_id: 1
        };

        pub struct SqlUtilsImplMock;

        #[async_trait]
        impl SqlUtils for SqlUtilsImplMock {
            async fn create_climb_route(&self, _climb_route: Json<ClimbRoute>) -> Result<i32, SqlError> {
                Err(SqlError::UnknownError)
            }
        }

        let sql_utils = SqlUtilsImplMock;

        let resp = create_climb_route_impl(&sql_utils, Json(climb_route)).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
}

