mod utils;

use utils::integ_tests_utils::IntegTestsUtilsImpl;
use crate::utils::integ_tests_utils::IntegTestsUtils;

#[test]
fn test_hello() {
    let utils = IntegTestsUtilsImpl { };

    let mut easy = utils.send_hello();

    let response_code = easy.response_code().unwrap();

    assert_eq!(response_code, 200);
}