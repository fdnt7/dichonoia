use core::fmt;

use crate::user::User;
use crate::{guild::UnavailableGuild, id::GuildId};
use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq; // codespell:ignore ser
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadyEvent {
    #[serde(rename = "v")]
    pub version: i32,
    pub user: User,

    /// Guilds the user is in
    #[serde(
        deserialize_with = "deserialise_guild_ids",
        serialize_with = "serialise_guild_ids_as_unavailable"
    )]
    // Internally, Discord's payload format for this field[^1] is an array of
    // *Unavailable Guild Object*s[^2]:
    // ```json
    // { "id": <string>, "unavailable": <bool> }
    // ```
    //
    // However, the `unavailable` field is guaranteed to be `true`[^3], so it can be
    // normalised to simply the `id` field as `GuildId`.
    //
    // [^1]: https://discord.com/developers/docs/events/gateway-events#ready-ready-event-fields
    // [^2]: https://discord.com/developers/docs/resources/guild#unavailable-guild-object
    // [^3]: https://discord.com/developers/docs/events/gateway-events#ready
    pub guilds: Vec<GuildId>,

    pub session_id: String,
    pub resume_gateway_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<[i32; 2]>,
    // Partial application object
    // pub application: Application
}

/// Custom deserializer: read a list of objects and keep only the `id` field.
fn deserialise_guild_ids<'de, D>(deserializer: D) -> Result<Vec<GuildId>, D::Error>
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
                out.push(g.id);
            }

            Ok(out)
        }
    }

    deserializer.deserialize_seq(GuildIdsVisitor)
}

/// Custom serializer: turn `Vec<GuildId>` back into the original
/// `[{"id": ..., "unavailable": true}, ...]` shape, so it still matches
/// Discord's payload format for serialisation.
fn serialise_guild_ids_as_unavailable<S>(
    guilds: &Vec<GuildId>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(guilds.len()))?;
    for &id in guilds {
        let tmp = UnavailableGuild {
            id,
            unavailable: true,
        };
        seq.serialize_element(&tmp)?;
    }
    seq.end()
}
