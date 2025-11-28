pub(crate) mod unavailable;

use serde::{Deserialize, Serialize};

use crate::id::GuildId;

/// <https://discord.com/developers/docs/resources/guild#guild-object>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guild {
    pub id: GuildId,
    // TODO: add other fields
}
