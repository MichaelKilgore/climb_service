use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct VerifyPhoneNumber {
    pub(crate) climb_user_id: i32,
    pub(crate) phone_number: String,
    pub(crate) code: String
}
