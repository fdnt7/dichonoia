use crate::ratelimit::{RateLimitError, RateLimiter};
use dichonoia_models::gateway::GatewayBot;
use reqwest::Client;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use serde::Serialize;
use serde::de::DeserializeOwned;

type Result<T, E = HttpError> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct HttpClient {
    token: String,
    http: Client,
    rate_limiter: RateLimiter,
}

#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("{0}")]
    RequestError(reqwest::Error),
    #[error("{0}")]
    JsonError(serde_json::Error),
    #[error("{0}")]
    RatelimitError(RateLimitError),
}

impl From<reqwest::Error> for HttpError {
    fn from(value: reqwest::Error) -> Self {
        Self::RequestError(value)
    }
}

impl From<serde_json::Error> for HttpError {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonError(value)
    }
}

impl From<RateLimitError> for HttpError {
    fn from(value: RateLimitError) -> Self {
        Self::RatelimitError(value)
    }
}

impl HttpClient {
    /// # Panics
    /// ...
    pub fn new(token: &str) -> Self {
        let mut headers: HeaderMap<HeaderValue> = HeaderMap::with_capacity(1);
        headers.insert(
            AUTHORIZATION,
            format!("Bot {token}")
                .parse()
                .expect("auth header must be correct"),
        );

        let http = Client::builder()
            .default_headers(headers)
            .build()
            .expect("TLS backend should be initialised");

        Self {
            http,
            token: String::from(token),
            rate_limiter: RateLimiter::new(),
        }
    }

    pub fn get_gateway_bot(&self) -> impl Future<Output = Result<GatewayBot>> {
        self.get("/gateway/bot")
    }

    async fn get<B: DeserializeOwned>(&self, path: &str) -> Result<B> {
        let resp = self.http.get(url(path)).send().await?.error_for_status()?;
        self.rate_limiter.check(&resp).await?;

        resp.json().await.map_err(HttpError::from)
    }

    async fn get_query<Q: Serialize + ?Sized + Send + Sync, B: DeserializeOwned>(
        &self,
        path: &str,
        query: &Q,
    ) -> Result<B> {
        let resp = self
            .http
            .get(url(path))
            .query(query)
            .send()
            .await?
            .error_for_status()?;
        self.rate_limiter.check(&resp).await?;

        resp.json().await.map_err(HttpError::from)
    }

    async fn post<Req: Serialize + ?Sized + Send + Sync, Resp: DeserializeOwned>(
        &self,
        path: &str,
        body: &Req,
    ) -> Result<Resp> {
        let resp = self
            .http
            .post(url(path))
            .json(body)
            .send()
            .await?
            .error_for_status()?;
        self.rate_limiter.check(&resp).await?;

        resp.json().await.map_err(HttpError::from)
    }
}

#[inline]
fn url(path: &str) -> String {
    format!("https://discord.com/api/v10{path}")
}
