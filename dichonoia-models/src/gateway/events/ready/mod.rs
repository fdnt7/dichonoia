pub mod shard_info;

use crate::user::User;
use crate::{gateway::events::ready::shard_info::ShardInfo, id::GuildId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadyEvent {
    #[serde(rename = "v")]
    pub version: u8,
    pub user: User,

    /// Guilds the user is in
    #[serde(with = "crate::guild::unavailable")]
    pub guilds: Vec<GuildId>,

    pub session_id: String,
    pub resume_gateway_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<ShardInfo>,
    // Partial application object
    // pub application: Application
}
