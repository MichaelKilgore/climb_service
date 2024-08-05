mod utils;

use serde_json::json;
use utils::integ_tests_utils::IntegTestsUtilsImpl;
use crate::utils::integ_tests_utils::IntegTestsUtils;

// https://www.twilio.com/en-us/blog/twilio-test-credentials-magic-numbers-html
#[test]
fn test_send_verification_code() {
    let utils = IntegTestsUtilsImpl { };

    let json_body = json!({
        "phone_number": "+15005550006"
    });
    let mut easy = utils.send_verification_code(json_body);
    
    let response_code = easy.response_code().unwrap();
    assert!(response_code == 201 || response_code == 429);
}
