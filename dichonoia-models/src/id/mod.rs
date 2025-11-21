pub mod marker;

use core::fmt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{marker::PhantomData, num::NonZeroU64};

use crate::id::marker::{Entity, Guild, User};

pub type GuildId = Snowflake<Guild>;
pub type UserId = Snowflake<User>;

#[derive(Debug, Clone, Copy)]
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
        struct Visitor<T: Entity>(PhantomData<fn(T) -> T>);

        impl<'de, T: Entity> serde::de::Visitor<'de> for Visitor<T> {
            type Value = Snowflake<T>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a non-zero u64 as a string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let raw: u64 = v.parse().map_err(E::custom)?;
                let inner = NonZeroU64::new(raw).ok_or_else(|| E::custom("ID must be non-zero"))?;

                Ok(Snowflake {
                    inner,
                    entity: PhantomData,
                })
            }
        }

        deserializer.deserialize_str(Visitor(PhantomData))
    }
}
