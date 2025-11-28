pub mod marker;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{marker::PhantomData, num::NonZeroU64};

use crate::id::marker::{Application, Entity, Guild, User};

pub type ApplicationId = Snowflake<Application>;
pub type GuildId = Snowflake<Guild>;
pub type UserId = Snowflake<User>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Snowflake<T: Entity> {
    inner: NonZeroU64,
    entity: PhantomData<fn(T) -> T>,
}

impl<T: Entity> Serialize for Snowflake<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.inner.get().to_string())
    }
}

impl<'de, T: Entity> Deserialize<'de> for Snowflake<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let n = value.parse().map_err(serde::de::Error::custom)?;
        Ok(Self::new_nonzero(n))
    }
}
