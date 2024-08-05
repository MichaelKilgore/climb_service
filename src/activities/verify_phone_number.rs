use actix_web::{HttpResponse, post};
use actix_web::web::Json;
use crate::errors::twilio_error::TwilioError;
use crate::model::verify_phone_number::VerifyPhoneNumber;
use crate::utils::general_utils::{GeneralConfig, GeneralUtils, GeneralUtilsImpl};
use crate::utils::sql_utils::{DbConfig, SqlUtils, SqlUtilsImpl};
use crate::utils::twilio_utils::{TwilioConfig, TwilioUtils};
use crate::utils::twilio_utils::TwilioUtilsImpl;

#[post("/verify-phone-number")]
pub async fn verify_phone_number(body: Json<VerifyPhoneNumber>) -> HttpResponse {
    let sql_utils = SqlUtilsImpl { db_config: DbConfig::new() };
    let twilio_util = TwilioUtilsImpl { twilio_config: TwilioConfig::new() };
    let general_util = GeneralUtilsImpl { general_config: GeneralConfig::new() };

    return verify_phone_number_impl(&sql_utils, &twilio_util, &general_util, body).await;
}

async fn verify_phone_number_impl<S, T, U>(sql_utils: &S, twilio_utils: &T, general_utils: &U, body: Json<VerifyPhoneNumber>) -> HttpResponse
    where
        S: SqlUtils,
        T: TwilioUtils,
        U: GeneralUtils
{
    if general_utils.is_request_a_test_request().await == false {
        // verify phone
        match twilio_utils.validate_verification_code(body.phone_number.clone(), body.code.clone()).await {
            Ok(..) => ..,
            Err(err) => {
                if err == TwilioError::IncorrectCode {
                    return HttpResponse::BadRequest().json(serde_json::json!({ "error": "Code is incorrect" }));
                }
                return HttpResponse::InternalServerError().json("Failed to validate phone number");
            }
        };
    } else {
        println!("Skipping twilio_utils call");
    }

    // update account
    return match sql_utils.set_phone_number_for_climb_user(body.climb_user_id, body.phone_number.clone()).await {
        Ok(..) => HttpResponse::Ok().json("{}"),
        Err(..) => HttpResponse::InternalServerError().body("Failed to update climb user")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use::actix_web::web::Json;
    use crate::errors::twilio_error::TwilioError;
    use crate::errors::sql_error::SqlError;

    #[actix_web::test]
    async fn verify_phone_number_success_test() {
        pub struct SqlUtilsImplMock;
        #[async_trait]
        impl SqlUtils for SqlUtilsImplMock {
            async fn set_phone_number_for_climb_user(&self, _climb_user_id: i32, _phone_number: String) -> Result<(), SqlError> {
                Ok(())
            }
        }
        let sql_utils = SqlUtilsImplMock;

        pub struct TwilioUtilsImplMock;
        #[async_trait]
        impl TwilioUtils for TwilioUtilsImplMock {
            async fn send_verification_code(&self, _phone_number: String) -> Result<(), TwilioError> {
                Ok(())
            }
        }
        let twilio_utils = TwilioUtilsImplMock;

        pub struct GeneralUtilsImplMock;
        #[async_trait]
        impl GeneralUtils for GeneralUtilsImplMock {
            async fn is_request_a_test_request(&self) -> bool {
                false
            }
        }
        let general_utils = GeneralUtilsImplMock;

        let body = VerifyPhoneNumber {
            climb_user_id: 1,
            phone_number: "+15005550006".to_string(),
            code: "148504".to_string()
        };

        let resp = verify_phone_number_impl(&sql_utils, &twilio_utils, &general_utils, Json(body)).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn verify_phone_number_test_request_test() {
        pub struct SqlUtilsImplMock;
        #[async_trait]
        impl SqlUtils for SqlUtilsImplMock {
            async fn set_phone_number_for_climb_user(&self, _climb_user_id: i32, _phone_number: String) -> Result<(), SqlError> {
                Ok(())
            }
        }
        let sql_utils = SqlUtilsImplMock;

        pub struct TwilioUtilsImplMock;
        #[async_trait]
        impl TwilioUtils for TwilioUtilsImplMock {
            async fn validate_verification_code(&self, _phone_number: String, _code: String) -> Result<(), TwilioError> {
                Err(TwilioError::UnknownError)
            }
        }
        let twilio_utils = TwilioUtilsImplMock;

        pub struct GeneralUtilsImplMock;
        #[async_trait]
        impl GeneralUtils for GeneralUtilsImplMock {
            async fn is_request_a_test_request(&self) -> bool {
                true
            }
        }
        let general_utils = GeneralUtilsImplMock;

        let body = VerifyPhoneNumber {
            climb_user_id: 1,
            phone_number: "+15005550006".to_string(),
            code: "148504".to_string()
        };

        let resp = verify_phone_number_impl(&sql_utils, &twilio_utils, &general_utils, Json(body)).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn verify_phone_number_twilio_says_incorrect_code_test() {
        pub struct SqlUtilsImplMock;
        #[async_trait]
        impl SqlUtils for SqlUtilsImplMock {
            async fn set_phone_number_for_climb_user(&self, _climb_user_id: i32, _phone_number: String) -> Result<(), SqlError> {
                Ok(())
            }
        }
        let sql_utils = SqlUtilsImplMock;

        pub struct TwilioUtilsImplMock;
        #[async_trait]
        impl TwilioUtils for TwilioUtilsImplMock { 
            async fn validate_verification_code(&self, _phone_number: String, _code: String) -> Result<(), TwilioError> {
                Err(TwilioError::IncorrectCode)
            }
        }
        let twilio_utils = TwilioUtilsImplMock;

        pub struct GeneralUtilsImplMock;
        #[async_trait]
        impl GeneralUtils for GeneralUtilsImplMock {
            async fn is_request_a_test_request(&self) -> bool {
                false
            }
        }
        let general_utils = GeneralUtilsImplMock;

        let body = VerifyPhoneNumber {
            climb_user_id: 1,
            phone_number: "+15005550006".to_string(),
            code: "148504".to_string()
        };

        let resp = verify_phone_number_impl(&sql_utils, &twilio_utils, &general_utils, Json(body)).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn verify_phone_number_twilio_says_unknown_error_test() {
        pub struct SqlUtilsImplMock;
        #[async_trait]
        impl SqlUtils for SqlUtilsImplMock {
            async fn set_phone_number_for_climb_user(&self, _climb_user_id: i32, _phone_number: String) -> Result<(), SqlError> {
                Ok(())
            }
        }
        let sql_utils = SqlUtilsImplMock;

        pub struct TwilioUtilsImplMock;
        #[async_trait]
        impl TwilioUtils for TwilioUtilsImplMock {
            async fn validate_verification_code(&self, _phone_number: String, _code: String) -> Result<(), TwilioError> {
                Err(TwilioError::UnknownError)
            }
        }
        let twilio_utils = TwilioUtilsImplMock;

        pub struct GeneralUtilsImplMock;
        #[async_trait]
        impl GeneralUtils for GeneralUtilsImplMock {
            async fn is_request_a_test_request(&self) -> bool {
                false
            }
        }
        let general_utils = GeneralUtilsImplMock;

        let body = VerifyPhoneNumber {
            climb_user_id: 1,
            phone_number: "+15005550006".to_string(),
            code: "148504".to_string()
        };

        let resp = verify_phone_number_impl(&sql_utils, &twilio_utils, &general_utils, Json(body)).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn verify_phone_number_sql_says_unknown_error_test() {
        pub struct SqlUtilsImplMock;
        #[async_trait]
        impl SqlUtils for SqlUtilsImplMock {
            async fn set_phone_number_for_climb_user(&self, _climb_user_id: i32, _phone_number: String) -> Result<(), SqlError> {
                Err(SqlError::UnknownError)
            }
        }
        let sql_utils = SqlUtilsImplMock;

        pub struct TwilioUtilsImplMock;
        #[async_trait]
        impl TwilioUtils for TwilioUtilsImplMock {
            async fn validate_verification_code(&self, _phone_number: String, _code: String) -> Result<(), TwilioError> {
                Ok(())
            }
        }
        let twilio_utils = TwilioUtilsImplMock;

        pub struct GeneralUtilsImplMock;
        #[async_trait]
        impl GeneralUtils for GeneralUtilsImplMock {
            async fn is_request_a_test_request(&self) -> bool {
                false
            }
        }
        let general_utils = GeneralUtilsImplMock;

        let body = VerifyPhoneNumber {
            climb_user_id: 1,
            phone_number: "+15005550006".to_string(),
            code: "148504".to_string()
        };

        let resp = verify_phone_number_impl(&sql_utils, &twilio_utils, &general_utils, Json(body)).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

}
