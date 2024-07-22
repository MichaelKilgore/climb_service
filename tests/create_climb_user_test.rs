use std::env;
use curl::easy::{Easy, List};
use regex::Regex;
use serde_json::Value;

#[test]
fn test_create_climb_user() {
    let mut easy = Easy::new();

    // set host
    let host = match env::var("SERVICE_URL") {
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
    easy.url(&format!("{host}/create-climb-user")).unwrap();
    easy.post(true).unwrap();

    // set authentication header
    let id_token = match env::var("ID_TOKEN") {
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
    let mut headers = List::new();
    if !id_token.is_empty() {
        headers.append(&format!("Authorization: Bearer {}", id_token)).unwrap();
        easy.http_headers(headers).unwrap();
    }

    // perform request
    easy.perform().unwrap();

    let mut response_body = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            response_body.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    let actual_json: Value = serde_json::from_slice(&response_body).unwrap();

    let response_code = easy.response_code().unwrap();
    
    // Check if the request was successful
    if response_code == 200 {
        println!("Request was successful!");
    } else {
        println!("Request failed!");
    }

    assert_eq!(response_code, 200);

    if let Some(user_name) = actual_json.get("user_name").and_then(Value::as_str) {
        let re = Regex::new(r"^user\d{20}$").unwrap();

        assert_eq!(true, re.is_match(user_name));
    } else {
        panic!("Failed to get user_name from json response");
    }

    if let Some(user_id) = actual_json.get("id").and_then(Value::as_i64) {
        let re = Regex::new(r"^\d").unwrap();

        assert_eq!(true, re.is_match(&*user_id.to_string()));
    } else {
        panic!("Failed to get id from json response");
    }
}