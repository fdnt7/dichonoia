mod parse;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{guild::Guild, id::GuildId};

/// Lazy-load for unavailable guild, guild became available, or user joined a new guild.
///
/// <div class="note">
/// During an outage, the guild may be marked as unavailable.
/// </div>
///
/// <https://discord.com/developers/docs/events/gateway-events#guild-create>
#[derive(Debug, Clone)]
pub enum GuildCreate {
    /// A [guild] object with extra data and create event source.
    ///
    /// [guild]: crate::guild::Guild
    Available {
        /// Guild object.
        guild: Guild,
        /// Source of the event.
        source: GuildCreateSource,
        /// Extra guild create event data.
        extra_data: GuildCreateExtraData,
    },
    /// A guild ID of the unavailable guild.
    Unavailable(GuildId),
}

/// Source of the Guild Create event.
///
/// This event can be sent in three different scenarios:
/// 1. When a user is initially connecting, to lazily load and backfill information for
///    all unavailable guilds sent in the [Ready] event.
///    Guilds that are unavailable due to an outage will send a [Guild Delete] event.
/// 2. When a Guild becomes available again to the client.
/// 3. When the current user joins a new Guild.
///
/// Scenario 1 and 2 are not distinguishable via the API.
///
/// [Ready]: crate::gateway::events::ready::Ready
/// [Guild Delete]: crate::gateway::events::guild_delete::GuildDelete
#[derive(Debug, Clone, Copy)]
pub enum GuildCreateSource {
    /// Either scenario 1 or 2.
    BecameAvailable,
    /// Scenario 3.
    Joined,
}

/// Extra data from the Guild Create event.
///
/// <div class="warning">
///
/// If your bot does not have the `GUILD_PRESENCES` [Gateway Intent], or if the guild has
/// over 75k members, members and presences returned in this event will only contain your
/// bot and users in voice channels.
/// </div>
///
/// <https://discord.com/developers/docs/events/gateway-events#guild-create-guild-create-extra-fields>
///
/// [Gateway Intent]: crate::gateway::GatewayIntents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildCreateExtraData {
    /// When this guild was joined at
    #[serde(with = "time::serde::rfc3339")]
    joined_at: OffsetDateTime,
    // TODO: add other fields
}

impl GuildCreate {
    #[must_use]
    pub const fn guild_id(&self) -> GuildId {
        match self {
            Self::Available { guild, .. } => guild.id,
            Self::Unavailable(guild_id) => *guild_id,
        }
    }
}
