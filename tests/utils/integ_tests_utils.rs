use std::env;
use curl::easy::{Easy, List};
use serde_json::Value;

pub trait IntegTestsUtils {
    fn get_host_url(&self) -> String;

    fn get_id_token(&self) -> String;

    fn send_create_climb_location_request(&self, json_body: Value) -> Easy;
    
    fn send_create_climb_user(&self) -> Easy;
    
    fn get_response_body(&self, easy: Easy) -> Value;
}

pub struct IntegTestsUtilsImpl;

impl IntegTestsUtils for IntegTestsUtilsImpl {
     fn get_host_url(&self) -> String {
        // set host
        return match env::var("SERVICE_URL") {
            Ok(value) => {
                value
            }
            Err(env::VarError::NotPresent) => {
                "http://localhost:8080".to_string()
            }
            Err(env::VarError::NotUnicode(_)) => {
                "http://localhost:8080".to_string()
            }
        };
    }

    fn get_id_token(&self) -> String {
        return match env::var("ID_TOKEN") {
            Ok(value) => {
                value
            }
            Err(env::VarError::NotPresent) => {
                "".to_string()
            }
            Err(env::VarError::NotUnicode(_)) => {
                "".to_string()
            }
        };
    }

    fn send_create_climb_location_request(&self, json_body: Value) -> Easy {
        let mut easy = Easy::new();

        let host = self.get_host_url();

        easy.url(&format!("{host}/create-climb-location")).unwrap();
        easy.post(true).unwrap();

        let id_token = self.get_id_token();
        let mut headers = List::new();
        if !id_token.is_empty() {
            headers.append(&format!("Authorization: Bearer {}", id_token)).unwrap();
        }
        headers.append("Content-Type: application/json").unwrap();
        easy.http_headers(headers).unwrap();

        easy.post_fields_copy(serde_json::to_string(&json_body).unwrap().as_bytes()).unwrap();

        easy.perform().unwrap();

        return easy;
    }

    fn send_create_climb_user(&self) -> Easy {
        let mut easy = Easy::new();        
        
        let host = self.get_host_url();
        
        easy.url(&format!("{host}/create-climb-user")).unwrap();
        easy.post(true).unwrap();

        let id_token = self.get_id_token();

        let mut headers = List::new();
        if !id_token.is_empty() {
            headers.append(&format!("Authorization: Bearer {}", id_token)).unwrap();
            easy.http_headers(headers).unwrap();
        }
        
        easy.perform().unwrap();

        return easy;
    }

    fn get_response_body(&self, mut easy: Easy) -> Value {
        let mut response_body = Vec::new();
        {
            let mut transfer = easy.transfer();
            transfer.write_function(|data| {
                response_body.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();
            transfer.perform().unwrap();
        }

        return serde_json::from_slice(&response_body).unwrap();
    }
    
}