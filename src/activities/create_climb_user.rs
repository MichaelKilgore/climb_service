use::actix_web::post;
use::actix_web::HttpResponse;
use crate::model::climb_user::ClimbUser;
use crate::utils::sql_utils::{DbConfig, SqlUtils, SqlUtilsImpl};
use rand::Rng;
use crate::errors::sql_error::SqlError;

#[post("/create-climb-user")]
pub async fn create_climb_user() -> HttpResponse {
    let sql_utils = SqlUtilsImpl { db_config: DbConfig::new() };

    return create_climb_user_impl(&sql_utils).await;
}

async fn create_climb_user_impl<S>(sql_utils: &S) -> HttpResponse
    where
        S: SqlUtils
{
    let mut rng = rand::thread_rng();

    let min = 10_u128.pow(19);
    let max = 10_u128.pow(20) - 1;

    let random_number = rng.gen_range(min..=max);

    let user_name = format!("user{}", random_number);

    let user = ClimbUser {
        user_name: user_name.clone(),
        phone_number: "".to_string(),
        status: "COMMENTOR".to_string(),
        moderator_comments: "".to_string()
    };

    let user_name_clone = user_name.clone();
    return match sql_utils.create_climb_user(user).await {
        Ok(id) => HttpResponse::Ok().json(serde_json::json!({ "id": id, "user_name": user_name_clone })),
        Err(err) => {
            if err == SqlError::PrimaryKeyAlreadyExists {
                return HttpResponse::Conflict().json("Insertion failed: user_name already exists");
            }
            return HttpResponse::InternalServerError().body("Failed to create climb user");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::to_bytes;
    use async_trait::async_trait;
    use serde_json::Value;
    use regex::Regex;

    #[actix_web::test]
    async fn test_create_climb_user_impl_success_test() {

        pub struct SqlUtilsImplMock;

        #[async_trait]
        impl SqlUtils for SqlUtilsImplMock {
            async fn create_climb_user(&self, _climb_user: ClimbUser) -> Result<i32, SqlError> {
                Ok(0)
            }
        }

        let sql_utils = SqlUtilsImplMock;

        let resp = crate::activities::create_climb_user::create_climb_user_impl(&sql_utils).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        let resp_body = to_bytes(resp.into_body()).await.unwrap();

        let actual_json: serde_json::Value = serde_json::from_slice(&resp_body).unwrap();

        if let Some(user_name) = actual_json.get("user_name").and_then(Value::as_str) {
            let re = Regex::new(r"^user\d{20}$").unwrap();

            assert_eq!(true, re.is_match(user_name));
        }
    }

    #[actix_web::test]
    async fn test_create_climb_user_impl_failure_user_name_already_exists_test() {
        pub struct SqlUtilsImplMock;

        #[async_trait]
        impl SqlUtils for SqlUtilsImplMock {
            async fn create_climb_user(&self, _climb_user: ClimbUser) -> Result<i32, SqlError> {
                Err(SqlError::PrimaryKeyAlreadyExists)
            }
        }

        let sql_utils = SqlUtilsImplMock;

        let resp = crate::activities::create_climb_user::create_climb_user_impl(&sql_utils).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::CONFLICT);
    }

    #[actix_web::test]
    async fn test_create_climb_user_impl_failure_unknown_error_test() {
        pub struct SqlUtilsImplMock;

        #[async_trait]
        impl SqlUtils for SqlUtilsImplMock {
            async fn create_climb_user(&self, _climb_user: ClimbUser) -> Result<i32, SqlError> {
                Err(SqlError::UnknownError)
            }
        }

        let sql_utils = SqlUtilsImplMock;

        let resp = crate::activities::create_climb_user::create_climb_user_impl(&sql_utils).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn test_create_climb_user_impl_fail_to_connect_to_db_test() {
        pub struct SqlUtilsImplMock;

        #[async_trait]
        impl SqlUtils for SqlUtilsImplMock {
            async fn create_climb_user(&self, _climb_user: ClimbUser) -> Result<i32, SqlError> {
                Err(SqlError::ConnectionError("Connection error.".to_string()))
            }
        }

        let sql_utils = SqlUtilsImplMock;

        let resp = crate::activities::create_climb_user::create_climb_user_impl(&sql_utils).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
}

