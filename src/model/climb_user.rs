use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ClimbUser {
    pub(crate) user_name: String,
    pub(crate) phone_number: String,
    pub(crate) status: String,
    pub(crate) moderator_comments: String
}
