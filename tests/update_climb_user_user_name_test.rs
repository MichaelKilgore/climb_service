use std::env;
use curl::easy::{Easy, List};
use regex::Regex;
use serde::Serialize;
use serde_json::Value;

/*
 creates a user, changes there name to poopyjr, 
 then changes there name back to what it was previously 
 set to so this test can run and succeed again.
 */
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
        easy.url(&format!("{host}/update-climb-user-user-name")).unwrap();
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
        }
        headers.append("Content-Type: application/json").unwrap();
        easy.http_headers(headers).unwrap();

        // set json body request
        #[derive(Serialize)]
        struct UserNameUpdate {
            user_name: String,
            new_user_name: String,
        }

        let update = UserNameUpdate {
            user_name: user_name.parse().unwrap(),
            new_user_name: "poopyjr".to_string(),
        };

        let json_data = serde_json::to_string(&update).unwrap();
        easy.post_fields_copy(json_data.as_bytes()).unwrap();

        // perform request
        easy.perform().unwrap();

        let response_code = easy.response_code().unwrap();

        // Check if the request was successful
        if response_code == 200 {
            println!("Request was successful!");
        } else {
            println!("Request failed!");
        }

        assert_eq!(response_code, 200);


        let update_back = UserNameUpdate {
            user_name: "poopyjr".to_string(),
            new_user_name: user_name.parse().unwrap(),
        };
        let json_data = serde_json::to_string(&update_back).unwrap();
        easy.post_fields_copy(json_data.as_bytes()).unwrap();

        easy.perform().unwrap();
        let response_code = easy.response_code().unwrap();

        // Check if the request was successful
        if response_code == 200 {
            println!("Request was successful!");
        } else {
            println!("Request failed!");
        }

        assert_eq!(response_code, 200);
    }
}