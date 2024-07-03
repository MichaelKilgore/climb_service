use std::env;
use curl::easy::{Easy, List};

#[test]
fn test_hello() {
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
    easy.url(&format!("{host}/hello")).unwrap();

    /*
    String::from_utf8(Command::new("sh")
                .arg("-c")
                .arg("gcloud auth print-identity-token")
                .output()
                .unwrap().stdout).unwrap()
     */
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
    headers.append(&format!("Authorization: Bearer {}", id_token)).unwrap();
    if !id_token.is_empty() {
        easy.http_headers(headers).unwrap();
    }

    // perform request
    eprintln!("THE HOST IS: {}", host);
    easy.perform().unwrap();

    let response_code = easy.response_code().unwrap();
    println!("Response code: {}", response_code);

    // Check if the request was successful
    if response_code == 200 {
        println!("Request was successful!");
    } else {
        println!("Request failed!");
    }

    assert_eq!(response_code, 200);
}