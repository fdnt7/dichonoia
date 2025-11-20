use serde::de::Error as DeError;
use serde::de::Unexpected;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Error as JsonError;
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum GatewayPayload {
    Dispatch(DispatchPayload), // 0
    Heartbeat,                 // 1
    Identify,                  // 2
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
    pub fn from_json(value: Value) -> Result<Self, JsonError> {
        let op_val = value.get("op").ok_or(JsonError::missing_field("op"))?;
        let op = i32::deserialize(op_val)?;

        match op {
            0 => Ok(Self::Dispatch(DispatchPayload::deserialize(value)?)),
            1 => Ok(Self::Heartbeat),
            2 => Ok(Self::Identify),
            3 => Ok(Self::PresenceUpdate),
            4 => Ok(Self::VoiceStateUpdate),
            6 => Ok(Self::Resume),
            7 => Ok(Self::Reconnect),
            8 => Ok(Self::RequestGuildMembers),
            9 => Ok(Self::InvalidSession),
            10 => Ok(Self::Hello(HelloPayload::deserialize(op_val)?)),
            11 => Ok(Self::HeartBeatACK),
            31 => Ok(Self::RequestSoundboardSounds),
            _ => Err(JsonError::invalid_value(
                Unexpected::Signed(op as i64),
                &"Not a valid opcode",
            )),
        }
    }

    pub fn to_json(self) -> Result<Value, JsonError> {
        let op = self.op();

        let mut value = match self {
            GatewayPayload::Dispatch(v) => serde_json::to_value(v)?,
            GatewayPayload::Hello(v) => serde_json::to_value(v)?,
            _ => Value::Object(Default::default()),
        };

        if let Value::Object(obj) = &mut value {
            obj.insert("op".into(), Value::Number(op.into()));
        }

        Ok(value)
    }

    pub fn op(&self) -> i32 {
        match self {
            GatewayPayload::Dispatch(_) => 0,
            GatewayPayload::Heartbeat => 1,
            GatewayPayload::Identify => 2,
            GatewayPayload::PresenceUpdate => 3,
            GatewayPayload::VoiceStateUpdate => 4,
            GatewayPayload::Resume => 6,
            GatewayPayload::Reconnect => 7,
            GatewayPayload::RequestGuildMembers => 8,
            GatewayPayload::InvalidSession => 9,
            GatewayPayload::Hello(_) => 10,
            GatewayPayload::HeartBeatACK => 11,
            GatewayPayload::RequestSoundboardSounds => 31,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum GatewayEvent {
    A,
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
    #[serde(rename = "d")]
    pub data: HelloPayloadData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelloPayloadData {
    heartbeat_interval: i32,
}
