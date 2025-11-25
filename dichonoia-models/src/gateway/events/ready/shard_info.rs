use std::num::NonZeroU16;

/// <https://discord.com/developers/docs/events/gateway#sharding>
#[derive(Debug, Clone, Copy)]
pub struct ShardInfo {
    id: u16,
    num: NonZeroU16,
}

/// Internally, Discord's payload format for the shard info is an
/// *array of two integers*[^1]: the shard ID and number of shards. We therefore
/// (de)serialize `ShardInfo` as a fixed two-element sequence, deserializing from the
/// positional tuple `(id, num)` so the wire format matches Discord's exactly.
///
/// [^1]: <https://discord.com/developers/docs/events/gateway-events#ready-ready-event-fields>
mod parse {
    use serde::{
        Deserialize,
        Deserializer,
        Serialize,
        Serializer,
        ser::SerializeSeq, // codespell:ignore ser
    };

    use crate::gateway::events::ready::shard_info::ShardInfo;

    impl Serialize for ShardInfo {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut seq = serializer.serialize_seq(Some(2))?;
            seq.serialize_element(&self.id)?;
            seq.serialize_element(&self.num)?;
            seq.end()
        }
    }

    impl<'de> Deserialize<'de> for ShardInfo {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let (id, num) = <_>::deserialize(deserializer)?;
            Ok(Self { id, num })
        }
    }
}
