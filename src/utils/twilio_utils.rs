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
}

pub struct TwilioUtilsImpl;

#[async_trait]
impl TwilioUtils for TwilioUtilsImpl {
    async fn send_verification_code(&self, phone_number: String) -> Result<(), TwilioError> {
        let account_service_id = env::var("TWILIO_ACCOUNT_SERVICE_ID").unwrap();
        let verify_service_id = env::var("TWILIO_VERIFY_SERVICE_ID").unwrap();
        let auth_token = env::var("TWILIO_AUTH_TOKEN").unwrap();
        
        let url = format!("https://verify.twilio.com/v2/Services/{verify_service_id}/Verifications");

        let client = Client::new();

        pub const STANDARD: GeneralPurpose = GeneralPurpose::new(&alphabet::STANDARD, PAD);

        let auth_value = format!("Basic {}", STANDARD.encode(format!("{}:{}", account_service_id, auth_token)));

        return match client.post(url)
            .header(AUTHORIZATION, auth_value)
            .form(&[("To", phone_number), ("Channel", "sms".parse().unwrap())])
            .send().await {
            Ok(resp) => {
                if resp.status() != StatusCode::CREATED {
                    eprintln!("Expected the status to be 201 for the following response: {:?}", resp.text().await);
                    return Err(TwilioError::UnknownError);
                }
                Ok(())
            },
            Err(err) => {
                eprintln!("Got an error sending post request to twilio: {err}");
                Err(TwilioError::UnknownError)
            }
        };
    }
    
}