use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::id::ApplicationId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PartialApplication {
    id: ApplicationId,
    flags: ApplicationFlags,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ApplicationFlags: u32 {
        const APPLICATION_AUTO_MODERATION_RULE_CREATE_BADGE = 1 << 6;
        const GATEWAY_PRESENCE = 1 << 12;
        const GATEWAY_PRESENCE_LIMITED = 1 << 13;
        const GATEWAY_GUILD_MEMBERS = 1 << 14;
        const GATEWAY_GUILD_MEMBERS_LIMITED = 1 << 15;
        const VERIFICATION_PENDING_GUILD_LIMIT = 1 << 16;
        const EMBEDDED = 1 << 17;
        const GATEWAY_MESSAGE_CONTENT = 1 << 18;
        const GATEWAY_MESSAGE_CONTENT_LIMITED = 1 << 19;
        const APPLICATION_COMMAND_BADGE = 1 << 23;
    }
}

impl Serialize for ApplicationFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.bits())
    }
}

impl<'de> Deserialize<'de> for ApplicationFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::from_bits(<_>::deserialize(deserializer)?)
            .ok_or_else(|| serde::de::Error::custom("invalid application flags"))
    }
}
