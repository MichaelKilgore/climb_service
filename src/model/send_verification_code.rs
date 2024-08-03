use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SendVerificationCode {
    pub(crate) phone_number: String
}
