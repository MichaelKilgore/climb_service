use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct UpdateClimbUserUserName {
    pub(crate) user_id: i32,
    pub(crate) new_user_name: String 
}