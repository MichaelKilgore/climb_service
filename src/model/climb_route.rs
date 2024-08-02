use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct ClimbRoute {
    pub(crate) name: String,
    pub(crate) grade: String,
    pub(crate) climb_location_id: i32,
    pub(crate) latitude: f64,
    pub(crate) longitude: f64,
    pub(crate) description: String,
    pub(crate) video_link: String,
    pub(crate) climb_user_id: i32
}