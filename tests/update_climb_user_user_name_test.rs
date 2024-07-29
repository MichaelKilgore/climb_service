mod utils;

use serde_json::json;
use utils::integ_tests_utils::IntegTestsUtilsImpl;
use crate::utils::integ_tests_utils::IntegTestsUtils;

#[test]
fn test_update_climb_user_user_name() {
    let utils = IntegTestsUtilsImpl { };

    // create user
    let easy = utils.send_create_climb_user();
    let response_body = utils.get_response_body(easy);
    let climb_user_id = response_body.get("climb_user_id").unwrap().as_i64().unwrap();
    let climb_user_name = response_body.get("user_name").unwrap().as_str().unwrap();

    // update user name
    let json_body = json!({
        "climb_user_id": climb_user_id as i32,
        "new_user_name": "poopyjr",
    });
    let mut easy = utils.send_update_climb_user_user_name(json_body);
    let response_code = easy.response_code().unwrap();
    assert_eq!(response_code, 200);

    // update username back to original username so this test can run again
    let json_body = json!({
        "climb_user_id": climb_user_id as i32,
        "new_user_name": climb_user_name,
    });
    let mut easy = utils.send_update_climb_user_user_name(json_body);
    let response_code = easy.response_code().unwrap();
    assert_eq!(response_code, 200);
}

#[test]
fn test_update_climb_user_user_name_fails_because_user_name_already_exists() {
    let utils = IntegTestsUtilsImpl { };

    // create user_1    
    let easy = utils.send_create_climb_user();
    let response_body = utils.get_response_body(easy);
    let climb_user_name_1 = response_body.get("user_name").unwrap().as_str().unwrap();
    
    // create user 2
    let easy = utils.send_create_climb_user();
    let response_body = utils.get_response_body(easy);
    let climb_user_id_2 = response_body.get("climb_user_id").unwrap().as_i64().unwrap();

    // attempt to change user 2's name to user 1's name
    let json_body = json!({
        "climb_user_id": climb_user_id_2 as i32,
        "new_user_name": climb_user_name_1,
    });
    let mut easy = utils.send_update_climb_user_user_name(json_body);
    let response_code = easy.response_code().unwrap();
    assert_eq!(response_code, 409);
}
