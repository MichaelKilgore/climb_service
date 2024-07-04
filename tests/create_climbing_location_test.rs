use std::env;
use curl::easy::{Easy, List};

#[test]
fn test_create_climbing_location() {
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
    easy.url(&format!("{host}/create-climbing-location")).unwrap();
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
    let json_data = r#"{"name":"Mount Everest Base Camp","profile_pic_location":"/images/mount-everest.jpg","description":"A popular trekking route in Nepal","address":"Sagarmatha National Park, Nepal"}"#;
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
}