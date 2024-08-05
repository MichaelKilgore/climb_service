use::actix_web::HttpResponse;
use actix_web::post;
use actix_web::web::Json;
use regex::Regex;
use crate::model::send_verification_code::SendVerificationCode;
use crate::utils::twilio_utils::{TwilioConfig, TwilioUtils, TwilioUtilsImpl};

#[post("/send-verification-code")]
pub async fn send_verification_code(body: Json<SendVerificationCode>) -> HttpResponse {
    let twilio_util = TwilioUtilsImpl { twilio_config: TwilioConfig::new() };
    
    return send_verification_code_impl(body, &twilio_util).await;
}

pub async fn send_verification_code_impl<S>(body: Json<SendVerificationCode>, twilio_util: &S) -> HttpResponse
    where
        S: TwilioUtils
{
    let phone_number = body.phone_number.clone();

    let re = Regex::new(r"^\+\d{11,15}$").unwrap();
    if !re.is_match(&*phone_number) {
        println!("Phone number received is not valid: {phone_number}");
        return HttpResponse::BadRequest().body("Invalid Phone Number");
    }

    return match twilio_util.send_verification_code(phone_number).await {
        Ok(..) => HttpResponse::Created().json("{}"),
        Err(..) => {
            HttpResponse::InternalServerError().body("Failed to send verification code")
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use::actix_web::web::Json;
    use crate::errors::twilio_error::TwilioError;

    #[actix_web::test]
    async fn send_verification_code_impl_success_test() {
        pub struct TwilioUtilsImplMock;

        #[async_trait]
        impl TwilioUtils for TwilioUtilsImplMock {
            async fn send_verification_code(&self, _phone_number: String) -> Result<(), TwilioError> {
                Ok(())
            }
        }
        
        let twilio_utils = TwilioUtilsImplMock;

        let body = SendVerificationCode {
            phone_number: "+15005550006".to_string()
        };

        let resp = crate::activities::send_verification_code::send_verification_code_impl(Json(body), &twilio_utils).await;
        
        assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);
    }

    #[actix_web::test]
    async fn send_verification_code_impl_invalid_phone_number_test() {
        pub struct TwilioUtilsImplMock;

        #[async_trait]
        impl TwilioUtils for TwilioUtilsImplMock {
            async fn send_verification_code(&self, _phone_number: String) -> Result<(), TwilioError> {
                Ok(())
            }
        }

        let twilio_utils = TwilioUtilsImplMock;

        let body = SendVerificationCode {
            phone_number: "+1abc".to_string()
        };

        let resp = crate::activities::send_verification_code::send_verification_code_impl(Json(body), &twilio_utils).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn send_verification_code_impl_internal_server_error_test() {
        pub struct TwilioUtilsImplMock;

        #[async_trait]
        impl TwilioUtils for TwilioUtilsImplMock {
            async fn send_verification_code(&self, _phone_number: String) -> Result<(), TwilioError> {
                Err(TwilioError::UnknownError)
            }
        }

        let twilio_utils = TwilioUtilsImplMock;

        let body = SendVerificationCode {
            phone_number: "+15005550006".to_string()
        };

        let resp = crate::activities::send_verification_code::send_verification_code_impl(Json(body), &twilio_utils).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
}
