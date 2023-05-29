use serde_derive::{Serialize, Deserialize};

#[derive(Insertable)]
pub struct UserModel{
    pub id: i64,
    pub usernaem: String,
    pub pass: String,
    pub email: String
}
