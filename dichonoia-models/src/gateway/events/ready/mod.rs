pub mod partial_application;
pub mod shard_info;

use crate::{
    gateway::events::ready::{partial_application::PartialApplication, shard_info::ShardInfo},
    id::GuildId,
    user::User,
};
use serde::{Deserialize, Serialize};

/// Contains the initial state information.
///
/// The ready event is dispatched when a client has completed the initial handshake with
/// the gateway (for new sessions). The ready event can be the largest and most complex
/// event the gateway will send, as it contains all the state required for a client to
/// begin interacting with the rest of the platform.
///
/// `guilds` are the guilds of which your bot is a member. They start out as unavailable
/// when you connect to the gateway. As they become available, your bot will be notified
/// via [Guild Create] events.
///
/// <https://discord.com/developers/docs/events/gateway-events#ready>
///
/// [Guild Create]: crate::gateway::events::guild_create::GuildCreate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ready {
    /// [API version](https://discord.com/developers/docs/reference#api-versioning-api-versions)
    #[serde(rename = "v")]
    pub version: u8,
    /// Information about the user including email
    pub user: User,
    /// Guilds the user is in
    #[serde(with = "crate::guild::unavailable")]
    pub guilds: Vec<GuildId>,
    /// Used for resuming connections
    pub session_id: String,
    /// Gateway URL for resuming connections
    pub resume_gateway_url: String,
    /// [Shard information] associated with this session, if sent when identifying
    ///
    /// [Shard information]: crate::gateway::events::ready::shard_info::ShardInfo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<ShardInfo>,
    /// Contains `id` and `flags`
    pub application: PartialApplication,
}
