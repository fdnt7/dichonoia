mod parse;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{guild::Guild, id::GuildId};

#[derive(Debug, Clone)]
pub enum GuildCreate {
    Available {
        guild: Guild,
        /// Source of the event.
        source: GuildCreateSource,
        extra_data: GuildCreateExtraData,
    },
    Unavailable(GuildId),
}

/// Source of the Guild Create event.
#[derive(Debug, Clone, Copy)]
pub enum GuildCreateSource {
    /// The current user has joined a new guild.
    Joined,
    /// A guild the current user was already a member of has become available again (e.g.,
    /// after an outage or temporary downtime).
    BecameAvailable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildCreateExtraData {
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
