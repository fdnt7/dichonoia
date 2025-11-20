use dichonoia_models::gateway::DispatchPayload;
use futures_util::StreamExt;
use thiserror::Error;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Error as TungsteniteError;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Encountered websocket error: {0}")]
    WebsocketError(TungsteniteError),
    #[error("Encountered json error: {0}")]
    JsonError(serde_json::Error),
}

impl From<TungsteniteError> for Error {
    fn from(value: TungsteniteError) -> Self {
        Self::WebsocketError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonError(value)
    }
}

pub struct GatewayStream {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl GatewayStream {
    pub async fn connect(token: &str) -> Result<Self> {
        let request = "wss://gateway.discord.gg/?v=10&encoding=json".into_client_request()?;
        let (stream, _response) = tokio_tungstenite::connect_async(request).await?;
        let gateway  =Self { stream };

        


        Ok(gateway)
    }

    pub async fn read_event(&mut self) -> Result<DispatchPayload> {
        // let quota = Quota::with_period(Duration::from_mins(1))
        //     .unwrap()
        //     .allow_burst(NonZeroU32::new(120u32).unwrap());

        let message = loop {
            if let Some(msg) = self.stream.next().await {
                break msg;
            }
        }?;

        let text = message.into_text()?;
        serde_json::from_str(&text).map_err(Error::from)
    }
}
