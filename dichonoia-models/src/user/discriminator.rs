use core::fmt;
use std::{
    fmt::{Display, Write},
    num::NonZeroU16,
};

use serde::{Deserialize, Deserializer, Serializer};

#[derive(Debug, Clone, Copy)]
pub struct Discriminator(pub NonZeroU16);

impl Display for Discriminator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.get() {
            ..=9 => f.write_str("000")?,
            10..=99 => f.write_str("00")?,
            100..=999 => f.write_char('0')?,
            _ => {}
        }

        self.0.fmt(f)
    }
}

/// The discriminator returned by the Discord API for users who migrated to Pomelo[^1].
///
/// [^1]: <https://dis.gd/usernames>
const POMELO: &str = "0";

pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Discriminator>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    if s == POMELO {
        return Ok(None);
    }

    if s.len() != 4 || !s.chars().all(|c| c.is_ascii_digit()) {
        return Err(serde::de::Error::custom(format!(
            "invalid discriminator '{s}': expected a 4-digit string"
        )));
    }

    let value = s.parse::<NonZeroU16>().map_err(|_| {
        serde::de::Error::custom(format!("invalid discriminator '{s}': cannot parse as u16"))
    })?;

    Ok(Some(Discriminator(value)))
}

#[expect(clippy::ref_option, clippy::trivially_copy_pass_by_ref)]
pub(super) fn serialize<S>(value: &Option<Discriminator>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(d) => serializer.serialize_str(&d.to_string()),
        None => serializer.serialize_str(POMELO),
    }
}
