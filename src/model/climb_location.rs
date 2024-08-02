use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct ClimbLocation {
    pub(crate) name: String,
    pub(crate) profile_pic_location: String,
    pub(crate) description: String,
    pub(crate) address: String,
    pub(crate) additional_info: String,
    pub(crate) moderator_comments: String
}
