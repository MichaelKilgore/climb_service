use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct ClimbingLocation {
    pub(crate) name: String,
    pub(crate) profile_pic_location: String,
    pub(crate) description: String,
    pub(crate) address: String,
}