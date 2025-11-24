//! Internally, Discord's payload format for the guild create event has three forms:
//! 1. When the current user joins a new guild:
//! ```json
//! { "id": <string>, <other guild fields>, <extra fields> }
//! ```
//! 2. When an unavailable guild becomes available:
//! ```json
//! { "id": <string>, <other guild fields>, "unavailable": false, <extra fields> }
//! ```
//! 3. When the current user joined a disabled guild during an investigation because of
//!    violations, or when Discord prepares a guild’s scheduled events at startup but
//!    subsequently fails to fetch the guild’s data, causing the guild to be returned as
//!    unavailable. This can also occur mid-runtime when the gateway reports that a
//!    previously unavailable guild is now available, even though it is not. Although
//!    highly unlikely to happen - this will be an *unavailable guild*[^1] object.
//!
//! Despite the overlap in the guild ID field, if all guild fields and only its ID are
//! flattened together, the ID will be over-counted by 1: first in from the guild, and
//! separately again to represent scenario #3. To avoid over-counting, A tagged union
//! can be used to represent the disjointure.
//!
//! [^1]: <https://discord.com/developers/docs/resources/guild#unavailable-guild-object>

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    gateway::events::{
        GuildCreate,
        guild_create::{GuildCreateExtraData, GuildCreateSource},
    },
    guild::{Guild, unavailable::UnavailableGuild},
};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum RawGuildCreate<G, M> {
    Available(AvailableGuildCreate<G, M>),
    Unavailable(UnavailableGuild),
}

#[derive(Serialize, Deserialize)]
struct AvailableGuildCreate<G, M> {
    #[serde(skip_serializing_if = "Option::is_none")]
    unavailable: Option<bool>,
    #[serde(flatten)]
    guild: G,
    #[serde(flatten)]
    metadata: M,
}

impl Serialize for GuildCreate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Available {
                guild,
                source,
                extra_data: metadata,
            } => RawGuildCreate::Available(AvailableGuildCreate {
                unavailable: match source {
                    GuildCreateSource::Joined => None,
                    GuildCreateSource::BecameAvailable => Some(false),
                },
                guild,
                metadata,
            }),
            Self::Unavailable(id) => RawGuildCreate::Unavailable((*id).into()),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for GuildCreate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = RawGuildCreate::<Guild, GuildCreateExtraData>::deserialize(deserializer)?;

        let obj = match value {
            RawGuildCreate::Available(AvailableGuildCreate {
                unavailable,
                guild,
                metadata,
            }) => {
                let source = match unavailable {
                    None => GuildCreateSource::Joined,
                    Some(false) => GuildCreateSource::BecameAvailable,
                    Some(true) => {
                        return Err(serde::de::Error::custom(
                            "`unavailable` must be absent or false for an available guild",
                        ));
                    }
                };
                Self::Available {
                    guild,
                    source,
                    extra_data: metadata,
                }
            }
            RawGuildCreate::Unavailable(unavailable_guild) => {
                Self::Unavailable(unavailable_guild.into())
            }
        };

        Ok(obj)
    }
}
