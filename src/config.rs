use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct Config {
    // channel id with -100
    pub channel_id: i64,
    pub bot_token: String,
    pub super_users: Vec<u64>,
}
