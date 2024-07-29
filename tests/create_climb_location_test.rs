mod utils;

use serde_json::json;
use utils::integ_tests_utils::IntegTestsUtilsImpl;
use crate::utils::integ_tests_utils::IntegTestsUtils;

#[test]
fn test_create_climb_location_success() {
    
    let utils = IntegTestsUtilsImpl { };

    let json_body = json!({
        "name": "Mount Everest Base Camp",
        "profile_pic_location": "/images/mount-everest.jpg",
        "description": "A popular trekking route in Nepal",
        "address": "Sagarmatha National Park, Nepal",
        "additional_info": "",
        "moderator_comments": ""
    });
    
    let mut easy = utils.send_create_climb_location_request(json_body);

    let response_code = easy.response_code().unwrap();

    assert_eq!(response_code, 200);
}

#[test]
fn test_create_climb_location_failure_location_doesnt_exist() {

    let utils = IntegTestsUtilsImpl { };

    let json_body = json!({
        "name": "Mount Everest Base Camp",
        "profile_pic_location": "/images/mount-everest.jpg",
        "description": "A popular trekking route in Nepal",
        "address": "asdf",
        "additional_info": "",
        "moderator_comments": ""
    });

    let mut easy = utils.send_create_climb_location_request(json_body);

    let response_code = easy.response_code().unwrap();

    assert_eq!(response_code, 400);   
}