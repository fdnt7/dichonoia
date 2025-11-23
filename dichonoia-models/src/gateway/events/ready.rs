use crate::id::GuildId;
use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadyEvent {
    #[serde(rename = "v")]
    pub version: i32,
    pub user: User,

    /// Guilds the user is in
    #[serde(with = "crate::guild::unavailable")]
    pub guilds: Vec<GuildId>,

    pub session_id: String,
    pub resume_gateway_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<[i32; 2]>,
    // Partial application object
    // pub application: Application
}
