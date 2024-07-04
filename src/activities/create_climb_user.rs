use::actix_web::post;
use::actix_web::HttpResponse;
use crate::model::climb_user::ClimbUser;
use crate::utils::sql_utils::{DbConfig, SqlUtils, SqlUtilsImpl};
use rand::Rng;

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

    let user_name = format!("bot{}", random_number);

    let user = ClimbUser {
        user_name: user_name.clone(),
        phone_number: "".to_string(),
        status: "COMMENTOR".to_string(),
        moderator_comments: "".to_string()
    };

    return match sql_utils.create_climb_user(user).await {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({ "user_name": user_name })),
        Err(err) => {
            println!("Error: {}", err);
            return HttpResponse::InternalServerError().body("Failed to create climb user");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::to_bytes;
    use async_trait::async_trait;
    use tokio_postgres::Error;
    use serde_json::Value;
    use regex::Regex;

    #[actix_web::test]
    async fn test_create_climb_user_impl_success_test() {

        pub struct SqlUtilsImplMock;

        #[async_trait]
        impl SqlUtils for SqlUtilsImplMock {
            async fn create_climb_user(&self, _climb_user: ClimbUser) -> Result<(), Error> {
                Ok(())
            }
        }

        let sql_utils = SqlUtilsImplMock;

        let resp = crate::activities::create_climb_user::create_climb_user_impl(&sql_utils).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        let resp_body = to_bytes(resp.into_body()).await.unwrap();

        let actual_json: serde_json::Value = serde_json::from_slice(&resp_body).unwrap();

        if let Some(user_name) = actual_json.get("user_name").and_then(Value::as_str) {
            let re = Regex::new(r"^bot\d{20}$").unwrap();
            
            assert_eq!(true, re.is_match(user_name));
        }
    }

}

