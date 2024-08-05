mod utils;

use serde_json::json;
use utils::integ_tests_utils::IntegTestsUtilsImpl;
use crate::utils::integ_tests_utils::IntegTestsUtils;

#[test]
fn test_verify_phone_number() {
    let utils = IntegTestsUtilsImpl { };

    // create user
    let easy = utils.send_create_climb_user();
    let response_body = utils.get_response_body(easy);

    let climb_user_id = response_body.get("climb_user_id").unwrap().as_i64().unwrap();

    /* verify that users phone number
       NOTE: This is a dummy code, and there is logic in the service itself to skip the twilio call,
             so we can still test this E2E */
    let json_body = json!({
        "climb_user_id": climb_user_id,
        "phone_number": "+15005550006",
        "code": "123456"
    });
    
    let mut easy = utils.verify_phone_number(json_body);

    let response_code = easy.response_code().unwrap();
    assert_eq!(response_code, 200);
}
