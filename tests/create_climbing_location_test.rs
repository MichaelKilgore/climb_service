use std::env;
use curl::easy::{Easy, List};

#[test]
fn test_create_climbing_location() {
    let mut easy = Easy::new();

    // set host
    let host = env::var("SERVICE_URL").unwrap();
    easy.url(&format!("{host}/hello")).unwrap();

    // set authentication header
    let id_token = env::var("ID_TOKEN").unwrap();
    let mut headers = List::new();
    headers.append(&format!("Authorization: Bearer {}", id_token)).unwrap();
    easy.http_headers(headers).unwrap();

    // perform request
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