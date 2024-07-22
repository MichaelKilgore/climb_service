use::actix_web::post;
use::actix_web::HttpResponse;
use actix_web::web::Json;
use crate::utils::sql_utils::{DbConfig, SqlUtils, SqlUtilsImpl};
use crate::errors::sql_error::SqlError;
use crate::model::update_climb_user_user_name::UpdateClimbUserUserName;

#[post("/update-climb-user-user-name")]
pub async fn update_climb_user_user_name(body: Json<UpdateClimbUserUserName>) -> HttpResponse {
    let sql_utils = SqlUtilsImpl { db_config: DbConfig::new() };

    return update_climb_user_user_name_impl(&sql_utils, body).await;
}

async fn update_climb_user_user_name_impl<S>(sql_utils: &S, body: Json<UpdateClimbUserUserName>) -> HttpResponse
    where
        S: SqlUtils
{
    return match sql_utils.update_climb_user_user_name(body.user_id, body.new_user_name.clone()).await {
        Ok(..) => HttpResponse::Ok().json("{}"),
        Err(err) => {
            if err == SqlError::PrimaryKeyAlreadyExists {
                return HttpResponse::Conflict().json("Insertion failed: user_name already exists");
            }
            return HttpResponse::InternalServerError().body("Failed to update climb users user_name");
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::climb_user::ClimbUser;
    use async_trait::async_trait;

    #[actix_web::test]
    async fn test_update_user_user_name_impl_success_test() {

        pub struct SqlUtilsImplMock;

        #[async_trait]
        impl SqlUtils for SqlUtilsImplMock {
            async fn create_climb_user(&self, _climb_user: ClimbUser) -> Result<i32, SqlError> {
                Ok(0)
            }
        }

        let body = UpdateClimbUserUserName {
            user_id: 1,
            new_user_name: "bill".to_string()
        };

        let sql_utils = SqlUtilsImplMock;

        let resp = update_climb_user_user_name_impl(&sql_utils, Json(body)).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }
}

