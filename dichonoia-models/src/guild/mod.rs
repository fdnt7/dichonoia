pub(crate) mod unavailable;

use serde::{Deserialize, Serialize};

use crate::id::GuildId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guild {
    pub id: GuildId,
    // TODO: add other fields
}
