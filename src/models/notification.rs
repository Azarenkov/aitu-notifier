// use serde::Serialize;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Notification {
    pub device_token: String,
    pub title: String,
    pub body: String,
}
