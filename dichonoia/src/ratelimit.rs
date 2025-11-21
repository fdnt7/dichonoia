use chrono::{DateTime, TimeZone, Utc};
use moka::Expiry;
use moka::future::Cache;
use reqwest::Response;
use std::str::FromStr;
use std::time::{Duration, Instant};

type Result<T, E = RateLimitError> = std::result::Result<T, E>;

#[derive(Debug, Clone, thiserror::Error)]
pub enum RateLimitError {
    /// Returned date time is when ratelimit reset occurs
    #[error("Ratelimit was hit")]
    Hit(DateTime<Utc>),
    #[error("Not all headers were supplied to construct a valid ratelimit state")]
    NotEnoughHeaders,
    #[error("Provided headers cannot be used to construct valid ratelimit state")]
    InvalidHeaders,
}

#[derive(Debug, Clone)]
pub(crate) struct RateLimiter {
    buckets: Cache<String, RateLimitState>,
}

impl RateLimiter {
    pub fn new() -> Self {
        let cache = Cache::builder().expire_after(RateLimitExpiry).build();
        Self { buckets: cache }
    }

    pub async fn check(&self, response: &Response) -> Result<()> {
        let bucket = get_header(response, "X-RateLimit-Bucket")?;

        // TODO: This implementation is wrong
        if let Some(mut bucket_state) = self.buckets.get(bucket).await {
            if bucket_state.remaining == 0 {
                Err(RateLimitError::Hit(bucket_state.reset))
            } else {
                bucket_state.remaining -= 1;
                self.buckets
                    .insert(String::from(bucket), bucket_state)
                    .await;
                Ok(())
            }
        } else {
            self.buckets
                .insert(
                    String::from(bucket),
                    RateLimitState::extract_state(response)?,
                )
                .await;

            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RateLimitState {
    _limit: i32,
    remaining: i32,
    reset: DateTime<Utc>,
}

impl RateLimitState {
    pub fn extract_state(response: &Response) -> Result<Self> {
        let limit: i32 = get_parsed_header(response, "X-RateLimit-Limit")?;
        let remaining: i32 = get_parsed_header(response, "X-RateLimit-Limit")?;
        let reset_stamp: f64 = get_parsed_header(response, "X-RateLimit-Reset")?;
        let reset = Utc.timestamp_opt(reset_stamp.floor() as i64, 0).unwrap();

        Ok(Self {
            _limit: limit,
            remaining,
            reset,
        })
    }
}

struct RateLimitExpiry;

impl Expiry<String, RateLimitState> for RateLimitExpiry {
    fn expire_after_create(
        &self,
        _key: &String,
        value: &RateLimitState,
        _created_at: Instant,
    ) -> Option<Duration> {
        let now = Utc::now();
        Some((value.reset - now).to_std().unwrap())
    }
}

#[inline]
fn get_header<'a>(response: &'a Response, header: &str) -> Result<&'a str> {
    response
        .headers()
        .get(header)
        .ok_or(RateLimitError::NotEnoughHeaders)?
        .to_str()
        .map_err(|_| RateLimitError::InvalidHeaders)
}

#[inline]
fn get_parsed_header<F: FromStr>(response: &Response, header: &str) -> Result<F> {
    get_header(response, header)?
        .parse()
        .map_err(|_| RateLimitError::InvalidHeaders)
}
