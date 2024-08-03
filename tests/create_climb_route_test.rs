mod utils;

use serde_json::json;
use utils::integ_tests_utils::IntegTestsUtilsImpl;
use crate::utils::integ_tests_utils::IntegTestsUtils;

#[test]
fn test_create_climb_route_success() {

    let utils = IntegTestsUtilsImpl { };

    //Create a climb location
    let climb_location_json = json!({
        "name": "Mount Everest Base Camp",
        "profile_pic_location": "/images/mount-everest.jpg",
        "description": "A popular trekking route in Nepal",
        "address": "Sagarmatha National Park, Nepal",
        "additional_info": "",
        "moderator_comments": ""
    });

    let easy = utils.send_create_climb_location_request(climb_location_json);
    let response_body = utils.get_response_body(easy);
    let climb_location_id = response_body.get("climb_location_id").unwrap().as_i64().unwrap();


    //Create a user
    let easy = utils.send_create_climb_user();

    let response_body = utils.get_response_body(easy);

    let climb_user_id = response_body.get("climb_user_id").unwrap().as_i64().unwrap();

    //Create route with previous IDs
    let json_body = json!({
        "name": "The Warmup Problem".to_string(),
        "grade": "V3".to_string(),
        "climb_location_id": climb_location_id,
        "latitude": 55.0,
        "longitude": -31.65,
        "description": "".to_string(),
        "video_link": "/videolink.com".to_string(),
        "climb_user_id": climb_user_id
    });

    let mut easy = utils.send_create_climb_route(json_body);

    let response_code = easy.response_code().unwrap();

    assert_eq!(response_code, 201);

}