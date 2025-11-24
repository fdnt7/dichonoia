//! Internally, This is Discord's payload format for an *unavailable guild* object[^1]:
//! ```json
//! { "id": <string>, "unavailable": <bool> }
//! ```
//!
//! However, the `unavailable` field is guaranteed to be `true`[^2], so it can be
//! normalised to simply the `id` field as `GuildId`.
//!
//! [^1]: <https://discord.com/developers/docs/resources/guild#unavailable-guild-object>
//! [^2]: <https://discord.com/developers/docs/events/gateway-events#ready>

use core::fmt;

use serde::{
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
    de::{SeqAccess, Visitor},
    ser::SerializeSeq, // codespell:ignore ser
};

use crate::id::GuildId;

#[derive(Serialize, Deserialize)]
pub struct UnavailableGuild {
    pub id: GuildId,
    pub unavailable: bool,
}

impl From<GuildId> for UnavailableGuild {
    fn from(value: GuildId) -> Self {
        Self {
            id: value,
            unavailable: true,
        }
    }
}

impl From<UnavailableGuild> for GuildId {
    fn from(value: UnavailableGuild) -> Self {
        value.id
    }
}

/// Custom serializer: turn `Vec<GuildId>` back into the original
/// `[{"id": ..., "unavailable": true}, ...]` shape, so it still matches
/// Discord's payload format for serialisation.
pub fn serialize<S>(guilds: &Vec<GuildId>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(guilds.len()))?;
    for &id in guilds {
        seq.serialize_element(&UnavailableGuild::from(id))?;
    }
    seq.end()
}

/// Custom deserializer: read a list of objects and keep only the `id` field.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<GuildId>, D::Error>
where
    D: Deserializer<'de>,
{
    struct GuildIdsVisitor;

    impl<'de> Visitor<'de> for GuildIdsVisitor {
        type Value = Vec<GuildId>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a sequence of guild objects with an `id` field")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut out = Vec::with_capacity(seq.size_hint().unwrap_or(0));

            while let Some(g) = seq.next_element::<UnavailableGuild>()? {
                out.push(g.into());
            }

            Ok(out)
        }
    }

    deserializer.deserialize_seq(GuildIdsVisitor)
}
