use std::num::NonZeroU16;

/// Shard information.
///
/// As apps grow and are added to an increasing number of guilds, some developers may find
/// it necessary to divide portions of their app's operations across multiple processes.
/// As such, the Gateway implements a method of user-controlled guild sharding which
/// allows apps to split events across a number of Gateway connections. Guild sharding is
/// entirely controlled by an app, and requires no state-sharing between separate
/// connections to operate. While all apps can enable sharding, it's not necessary for
/// apps in a smaller number of guilds.
///
/// <div class="warning">
///
/// Each shard can only support a maximum of 2500 guilds, and apps that are in 2500+
/// guilds *must* enable sharding.
///
/// </div>
///
/// To enable sharding on a connection, the app should send the `shard` array in the
/// [Identify] payload. The first item in this array should be the zero-based integer
/// value of the current shard, while the second represents the total number of shards.
///
/// <div class="note">
///
/// The
/// [Get Gateway Bot](../../../../../dichonoia/http/struct.HttpClient.html#method.get_gateway_bot)
/// endpoint provides a recommended number of shards for your app
/// in the `shards` field
/// </div>
///
/// To calculate which events will be sent to which shard, the following formula can be
/// used:
///
/// ## Sharding Formula
///
/// ```python
/// shard_id = (guild_id >> 22) % num_shards
/// ```
///
/// As an example, if you wanted to split the connection between three shards, you'd use
/// the following values for `shard` for each connection: `[0, 3]`, `[1, 3]`, and
/// `[2, 3]`.
///
/// <div class="note">
///
/// Gateway events that do not contain a `guild_id` will only be sent to the first shard
/// (`shard_id: 0`). This includes Direct Message (DM), subscription and entitlement
/// events.
/// </div>
///
/// Note that `num_shards` does not relate to (or limit) the total number of potential
/// sessions. It is only used for *routing* traffic. As such, sessions do not have to be
/// identified in an evenly-distributed manner when sharding. You can establish multiple
/// sessions with the same `[shard_id, num_shards]`, or sessions with different
/// `num_shards` values. This allows you to create sessions that will handle more or less
/// traffic for more fine-tuned load balancing, or to orchestrate "zero-downtime"
/// scaling/updating by handing off traffic to a new deployment of sessions with a higher
/// or lower `num_shards` count that are prepared in parallel.
///
/// ## Max Concurrency
///
/// If you have multiple shards, you may start them concurrently based on the
/// [`max_concurrency`] value returned to you on session start. Which shards you can start
/// concurrently are assigned based on a key for each shard. The rate limit key for a
/// given shard can be computed with
///
/// ```python
/// rate_limit_key = shard_id % max_concurrency
/// ```
///
/// This puts your shards into "buckets" of `max_concurrency` size. When you start your
/// bot, you may start up to `max_concurrency` shards at a time, and you must start them
/// by "bucket" **in order**. To explain another way, let's say you have 16 shards, and
/// your `max_concurrency` is 16:
///
/// ```text
/// shard_id: 0, rate limit key (0 % 16): 0
/// shard_id: 1, rate limit key (1 % 16): 1
/// shard_id: 2, rate limit key (2 % 16): 2
/// shard_id: 3, rate limit key (3 % 16): 3
/// shard_id: 4, rate limit key (4 % 16): 4
/// shard_id: 5, rate limit key (5 % 16): 5
/// shard_id: 6, rate limit key (6 % 16): 6
/// shard_id: 7, rate limit key (7 % 16): 7
/// shard_id: 8, rate limit key (8 % 16): 8
/// shard_id: 9, rate limit key (9 % 16): 9
/// shard_id: 10, rate limit key (10 % 16): 10
/// shard_id: 11, rate limit key (11 % 16): 11
/// shard_id: 12, rate limit key (12 % 16): 12
/// shard_id: 13, rate limit key (13 % 16): 13
/// shard_id: 14, rate limit key (14 % 16): 14
/// shard_id: 15, rate limit key (15 % 16): 15
/// ```
///
/// You may start all 16 of your shards at once, because each has a `rate_limit_key`
/// which fills the bucket of 16 shards. However, let's say you had 32 shards:
///
/// ```text
/// shard_id: 0, rate limit key (0 % 16): 0
/// shard_id: 1, rate limit key (1 % 16): 1
/// shard_id: 2, rate limit key (2 % 16): 2
/// shard_id: 3, rate limit key (3 % 16): 3
/// shard_id: 4, rate limit key (4 % 16): 4
/// shard_id: 5, rate limit key (5 % 16): 5
/// shard_id: 6, rate limit key (6 % 16): 6
/// shard_id: 7, rate limit key (7 % 16): 7
/// shard_id: 8, rate limit key (8 % 16): 8
/// shard_id: 9, rate limit key (9 % 16): 9
/// shard_id: 10, rate limit key (10 % 16): 10
/// shard_id: 11, rate limit key (11 % 16): 11
/// shard_id: 12, rate limit key (12 % 16): 12
/// shard_id: 13, rate limit key (13 % 16): 13
/// shard_id: 14, rate limit key (14 % 16): 14
/// shard_id: 15, rate limit key (15 % 16): 15
/// shard_id: 16, rate limit key (16 % 16): 0
/// shard_id: 17, rate limit key (17 % 16): 1
/// shard_id: 18, rate limit key (18 % 16): 2
/// shard_id: 19, rate limit key (19 % 16): 3
/// shard_id: 20, rate limit key (20 % 16): 4
/// shard_id: 21, rate limit key (21 % 16): 5
/// shard_id: 22, rate limit key (22 % 16): 6
/// shard_id: 23, rate limit key (23 % 16): 7
/// shard_id: 24, rate limit key (24 % 16): 8
/// shard_id: 25, rate limit key (25 % 16): 9
/// shard_id: 26, rate limit key (26 % 16): 10
/// shard_id: 27, rate limit key (27 % 16): 11
/// shard_id: 28, rate limit key (28 % 16): 12
/// shard_id: 29, rate limit key (29 % 16): 13
/// shard_id: 30, rate limit key (30 % 16): 14
/// shard_id: 31, rate limit key (31 % 16): 15
/// ```
///
/// In this case, you must start the shard buckets **in "order"**. That means that you can
/// start shard 0 -> shard 15 concurrently, and then you can start shard 16 -> shard 31.
///
/// # Sharding for Large Bots
///
/// If your bot is in more than 150,000 guilds, there are some additional considerations
/// you must take around sharding. Discord will migrate your bot to large bot sharding
/// when it starts to get near the large bot sharding threshold. The bot owner(s) will
/// receive a system DM and email confirming this move has completed as well as what shard
/// number has been assigned.
///
/// The number of shards you run must be a multiple of the shard number provided when
/// reaching out to you. If you attempt to start your bot with an invalid number of
/// shards, your Gateway connection will close with a `4010` Invalid Shard close code.
///
/// The
/// [Get Gateway Bot endpoint](../../../../../dichonoia/http/struct.HttpClient.html#method.get_gateway_bot)
/// will always return the correct amount of shards, so if you're already using this
/// endpoint to determine your number of shards, you shouldn't require any changes.
///
/// The session start limit for these bots will also be increased from 1000 to
/// `max(2000, (guild_count / 1000) * 5)` per day. You also receive an increased
/// `max_concurrency`, the number of
/// [shards you can concurrently start](crate::gateway::SessionStartLimit).
///
/// <https://discord.com/developers/docs/events/gateway#sharding>
///
/// [Identify]: crate::gateway::IdentifyPayload
/// [`max_concurrency`]: crate::gateway::SessionStartLimit
#[derive(Debug, Clone, Copy)]
pub struct ShardInfo {
    /// Shard ID
    pub id: u16,
    /// Number of shards
    pub num: NonZeroU16,
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
