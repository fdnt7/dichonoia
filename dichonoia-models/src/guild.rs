use serde::{Deserialize, Serialize};

use crate::id::GuildId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnavailableGuild {
    pub id: GuildId,
    pub unavailable: bool,
}
