pub mod events;

use crate::gateway::events::{GuildCreate, ReadyEvent};
use bitflags::bitflags;
use serde::Deserializer;
use serde::de::Error as DeError;
use serde::de::Unexpected;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Error as JsonError;
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum GatewayPayload {
    Dispatch(DispatchPayload), // 0
    Heartbeat,                 // 1
    Identify(IdentifyPayload), // 2
    PresenceUpdate,            // 3
    VoiceStateUpdate,          // 4
    Resume,                    // 6
    Reconnect,                 // 7
    RequestGuildMembers,       // 8
    InvalidSession,            // 9
    Hello(HelloPayload),       // 10
    HeartBeatACK,              // 11
    RequestSoundboardSounds,   // 31
}

impl GatewayPayload {
    /// # Errors
    ///
    /// ...
    pub fn from_json(value: Value) -> Result<Self, JsonError> {
        let op_val = value
            .get("op")
            .ok_or_else(|| JsonError::missing_field("op"))?;
        let op = i32::deserialize(op_val)?;

        match op {
            0 => Ok(Self::Dispatch(DispatchPayload::deserialize(value)?)),
            1 => Ok(Self::Heartbeat),
            2 => Ok(Self::Identify(Self::deserialize_data(&value)?)),
            3 => Ok(Self::PresenceUpdate),
            4 => Ok(Self::VoiceStateUpdate),
            6 => Ok(Self::Resume),
            7 => Ok(Self::Reconnect),
            8 => Ok(Self::RequestGuildMembers),
            9 => Ok(Self::InvalidSession),
            10 => Ok(Self::Hello(Self::deserialize_data(&value)?)),
            11 => Ok(Self::HeartBeatACK),
            31 => Ok(Self::RequestSoundboardSounds),
            _ => Err(JsonError::invalid_value(
                Unexpected::Signed(i64::from(op)),
                &"Not a valid opcode",
            )),
        }
    }

    fn deserialize_data<'de, D: Deserialize<'de>>(value: &'de Value) -> Result<D, JsonError> {
        let data = value
            .get("d")
            .ok_or_else(|| JsonError::missing_field("d"))?;
        D::deserialize(data)
    }

    /// # Panics
    ///
    /// ...
    ///
    /// # Errors
    ///
    /// ...
    pub fn to_json(self) -> Result<Value, JsonError> {
        let op = self.op();

        let mut value = if let Self::Dispatch(v) = self {
            serde_json::to_value(v)?
        } else {
            let data = match self {
                Self::Identify(v) => serde_json::to_value(v)?,
                Self::Hello(v) => serde_json::to_value(v)?,
                _ => Value::Null,
            };

            if matches!(data, Value::Object(_)) {
                let mut map = serde_json::Map::with_capacity(2);
                map.insert(String::from("d"), data);

                Value::Object(map)
            } else {
                Value::Object(serde_json::Map::with_capacity(1))
            }
        };

        if let Value::Object(obj) = &mut value {
            obj.insert("op".into(), Value::Number(op.into()));
        } else {
            panic!("Expected Value::Object, got {value:?}");
        }

        Ok(value)
    }

    #[must_use]
    pub const fn op(&self) -> i32 {
        match self {
            Self::Dispatch(_) => 0,
            Self::Heartbeat => 1,
            Self::Identify(_) => 2,
            Self::PresenceUpdate => 3,
            Self::VoiceStateUpdate => 4,
            Self::Resume => 6,
            Self::Reconnect => 7,
            Self::RequestGuildMembers => 8,
            Self::InvalidSession => 9,
            Self::Hello(_) => 10,
            Self::HeartBeatACK => 11,
            Self::RequestSoundboardSounds => 31,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "t", content = "d", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GatewayEvent {
    Ready(ReadyEvent),
    GuildCreate(GuildCreate),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispatchPayload {
    #[serde(flatten)]
    pub data: Option<GatewayEvent>,
    #[serde(rename = "s")]
    pub sequence: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelloPayload {
    pub heartbeat_interval: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentifyPayload {
    pub token: String,
    pub properties: IdentifyProperties,
    #[serde(default)]
    pub compress: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_threshold: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<[i32; 2]>,
    pub intents: GatewayIntents,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentifyProperties {
    pub os: String,
    pub browser: String,
    pub device: String,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct GatewayIntents: u32 {
        const GUILDS = 1 << 0;
        const GUILD_MEMBERS = 1 << 1;
        const GUILD_MODERATION = 1 << 2;
        const GUILD_EXPRESSIONS = 1 << 3;
        const GUILD_INTEGRATIONS = 1 << 4;
        const GUILD_WEBHOOKHS = 1 << 5;
        const GUILD_INVITES = 1 << 6;
        const GUILD_VOICE_STATES = 1 << 7;
        const GUILD_PRESENCES = 1 << 8;
        const GUILD_MESSAGES = 1 << 9;
        const GUILD_MESSAGE_REACTIONS = 1 << 10;
        const GUILD_MESSAGE_TYPING = 1 << 11;
        const DIRECT_MESSAGES = 1 << 12;
        const DIRECT_MESSAGE_REACTIONS = 1 << 13;
        const DIRECT_MESSAGE_TYPING = 1 << 14;
        const MESSAGE_CONTENT = 1 << 15;
        const GUILD_SCHEDULED_EVENTS = 1 << 16;
        const AUTO_MODERATION_CONFIGURATION = 1 << 20;
        const AUTO_MODERATION_EXECUTION = 1 << 21;
        const GUILD_MESSAGE_POLLS = 1 << 24;
        const DIRECT_MESSAGE_POLLS = 1 << 25;
    }
}

impl Serialize for GatewayIntents {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.bits())
    }
}

impl<'de> Deserialize<'de> for GatewayIntents {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::from_bits(<_>::deserialize(deserializer)?)
            .ok_or_else(|| serde::de::Error::custom("invalid application flags"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayBot {
    pub url: String,
    pub shards: i32,
    pub session_start_limit: SessionStartLimit,
}

/// Session Start Limit Object
///
/// <https://discord.com/developers/docs/events/gateway#session-start-limit-object>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStartLimit {
    /// Total number of session starts the current user is allowed
    pub total: i32,
    /// Remaining number of session starts the current user is allowed
    pub remaining: i32,
    /// Number of milliseconds after which the limit resets
    pub reset_after: i64,
    /// Number of identify requests allowed per 5 seconds
    pub max_concurrency: i32,
}
