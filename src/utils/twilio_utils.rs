use std::env;
use base64::{alphabet, Engine};
use reqwest::header::AUTHORIZATION;
use async_trait::async_trait;
use base64::engine::general_purpose::PAD;
use base64::engine::GeneralPurpose;
use reqwest::{Client, StatusCode};
use crate::errors::twilio_error::TwilioError;

#[async_trait]
pub trait TwilioUtils: Send + Sync {
    async fn send_verification_code(&self, _phone_number: String) -> Result<(), TwilioError> {
        Ok(())
    }

    async fn validate_verification_code(&self, _phone_number: String, _code: String) -> Result<(), TwilioError> {
        Ok(())
    }
}

#[derive(Clone)]
pub struct TwilioConfig {
    account_service_id: String,
    verify_service_id: String,
    auth_token: String,
}

impl TwilioConfig {
    pub(crate) fn new() -> Self {
        TwilioConfig {
            account_service_id: env::var("TWILIO_ACCOUNT_SERVICE_ID").unwrap(),
            verify_service_id: env::var("TWILIO_VERIFY_SERVICE_ID").unwrap(),
            auth_token: env::var("TWILIO_AUTH_TOKEN").unwrap()
        }
    }
}

pub struct TwilioUtilsImpl {
    pub(crate) twilio_config: TwilioConfig
}

#[async_trait]
impl TwilioUtils for TwilioUtilsImpl {
    async fn send_verification_code(&self, phone_number: String) -> Result<(), TwilioError> {
        let url = format!("https://verify.twilio.com/v2/Services/{0}/Verifications", self.twilio_config.verify_service_id);

        let client = Client::new();

        pub const STANDARD: GeneralPurpose = GeneralPurpose::new(&alphabet::STANDARD, PAD);

        let auth_value = format!("Basic {}", STANDARD.encode(format!("{}:{}", self.twilio_config.account_service_id, 
                                                                     self.twilio_config.auth_token)));

        return match client.post(url)
            .header(AUTHORIZATION, auth_value)
            .form(&[("To", phone_number), ("Channel", "sms".parse().unwrap())])
            .send().await {
            Ok(resp) => {
                if resp.status() == StatusCode::CREATED {
                    return Ok(())
                }
                if resp.status() == StatusCode::TOO_MANY_REQUESTS {
                    return Err(TwilioError::TooManyRequests);
                }
                eprintln!("Unknown response received: {:?}", resp.text().await);
                return Err(TwilioError::UnknownError);
            },
            Err(err) => {
                eprintln!("Got an error sending post request to twilio: {err}");
                Err(TwilioError::UnknownError)
            }
        };
    }

    async fn validate_verification_code(&self, phone_number: String, code: String) -> Result<(), TwilioError> {
        let url = format!("https://verify.twilio.com/v2/Services/{0}/VerificationCheck", self.twilio_config.verify_service_id);

        let client = Client::new();

        pub const STANDARD: GeneralPurpose = GeneralPurpose::new(&alphabet::STANDARD, PAD);

        let auth_value = format!("Basic {}", STANDARD.encode(format!("{}:{}", self.twilio_config.account_service_id,
                                                                     self.twilio_config.auth_token)));

        return match client.post(url)
            .header(AUTHORIZATION, auth_value)
            .form(&[("To", phone_number), ("Code", code)])
            .send().await {
            Ok(resp) => {
                if resp.status() != StatusCode::OK {
                    eprintln!("Expected the status to be 200 for the following response: {:?}", resp.text().await);
                    return Err(TwilioError::UnknownError);
                }
                Ok(())
            },
            Err(err) => {
                eprintln!("Got an error sending request to twilio: {err}");
                Err(TwilioError::UnknownError)
            }
        };
    }
    
}
