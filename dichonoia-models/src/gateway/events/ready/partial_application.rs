use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::id::ApplicationId;

/// Partial [application object].
///
/// [application object]: crate::application::Application
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PartialApplication {
    /// ID of the app
    id: ApplicationId,
    /// App's public [flags]
    ///
    /// [flags]: crate::gateway::events::ready::partial_application::ApplicationFlags
    flags: ApplicationFlags,
}

bitflags! {
    /// <https://discord.com/developers/docs/resources/application#application-object-application-flags>
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ApplicationFlags: u32 {
        /// Indicates if an app uses the
        /// [Auto Moderation API](https://discord.com/developers/docs/resources/auto-moderation)
        const APPLICATION_AUTO_MODERATION_RULE_CREATE_BADGE = 1 << 6;
        /// Intent required for bots in **100 or more servers** to receive
        /// [`presence_update` events](https://discord.com/developers/docs/events/gateway-events#presence-update)
        const GATEWAY_PRESENCE = 1 << 12;
        /// Intent required for bots in under 100 servers to receive,
        /// [`presence_update` events](https://discord.com/developers/docs/events/gateway-events#presence-update),
        /// found on the **Bot** page in your app's settings
        const GATEWAY_PRESENCE_LIMITED = 1 << 13;
        /// Intent required for bots in **100 or more servers** to receive member-related
        /// events like `guild_member_add`. See the list of member-related events
        /// [under `GUILD_MEMBERS`](https://discord.com/developers/docs/events/gateway#list-of-intents)
        const GATEWAY_GUILD_MEMBERS = 1 << 14;
        /// Intent required for bots in under 100 servers to receive member-related events
        /// like `guild_member_add`, found on the **Bot** page in your app's settings. See
        /// the list of member-related events
        /// [under `GUILD_MEMBERS`](https://discord.com/developers/docs/events/gateway#list-of-intents)
        const GATEWAY_GUILD_MEMBERS_LIMITED = 1 << 15;
        /// Indicates unusual growth of an app that prevents verification
        const VERIFICATION_PENDING_GUILD_LIMIT = 1 << 16;
        /// Indicates if an app is embedded within the Discord client
        /// (currently unavailable publicly)
        const EMBEDDED = 1 << 17;
        /// Intent required for bots in **100 or more servers** to receive
        /// [message content](https://support-dev.discord.com/hc/en-us/articles/4404772028055)
        const GATEWAY_MESSAGE_CONTENT = 1 << 18;
        /// Intent required for bots in under 100 servers to receive
        /// [message content](https://support-dev.discord.com/hc/en-us/articles/4404772028055),
        /// found on the **Bot** page in your app's settings
        const GATEWAY_MESSAGE_CONTENT_LIMITED = 1 << 19;
        /// Indicates if an app has registered global
        /// [application commands](https://discord.com/developers/docs/interactions/application-commands)
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
