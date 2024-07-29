mod utils;

use regex::Regex;
use utils::integ_tests_utils::IntegTestsUtilsImpl;
use crate::utils::integ_tests_utils::IntegTestsUtils;

#[test]
fn test_create_climb_user() {
    let utils = IntegTestsUtilsImpl { };

    let mut easy = utils.send_create_climb_user();

    let response_code = easy.response_code().unwrap();

    assert_eq!(response_code, 200);
    
    let response_body = utils.get_response_body(easy);
    
    let user_name = response_body.get("user_name").unwrap().as_str().unwrap();
    let re = Regex::new(r"^user\d{20}$").unwrap();
    assert_eq!(true, re.is_match(&user_name));

    let climb_user_id = response_body.get("climb_user_id").unwrap().as_i64().unwrap();
    let re = Regex::new(r"^\d").unwrap();
    assert_eq!(true, re.is_match(&*climb_user_id.to_string()));
}