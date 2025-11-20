use dichonoia_models::gateway::{
    GatewayIntents, GatewayPayload, IdentifyPayload, IdentifyProperties,
};
use futures_util::{SinkExt, StreamExt};
use thiserror::Error;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::{Error as TungsteniteError, Message};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub type Result<T, E = Error> = std::result::Result<T, E>;
type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

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
    stream: WsStream,
    heartbeat_interval: i32,
}

impl GatewayStream {
    pub async fn connect(token: &str, intents: GatewayIntents) -> Result<Self> {
        let request = "wss://gateway.discord.gg/?v=10&encoding=json".into_client_request()?;
        let (mut stream, _response) = tokio_tungstenite::connect_async(request).await?;

        let GatewayPayload::Hello(hello_payload) = Self::read_from_stream(&mut stream).await?
        else {
            panic!("Expected Hello from discord");
        };

        let identify = IdentifyPayload {
            intents,
            properties: IdentifyProperties {
                os: String::from(std::env::consts::OS),
                device: format!("dichonoia/{}", env!("CARGO_PKG_VERSION")),
                browser: format!("dichonoia/{}", env!("CARGO_PKG_VERSION")),
            },
            token: String::from(token),
            compress: false,
            shard: None,
            large_threshold: None,
        };
        Self::write_to_stream(&mut stream, GatewayPayload::Identify(identify)).await?;

        Ok(Self {
            stream,
            heartbeat_interval: hello_payload.heartbeat_interval,
        })
    }

    pub fn read_payload(&mut self) -> impl Future<Output = Result<GatewayPayload>> {
        Self::read_from_stream(&mut self.stream)
    }

    async fn read_from_stream(stream: &mut WsStream) -> Result<GatewayPayload> {
        // let quota = Quota::with_period(Duration::from_mins(1))
        //     .unwrap()
        //     .allow_burst(NonZeroU32::new(120u32).unwrap());

        let message = loop {
            if let Some(msg) = stream.next().await {
                break msg;
            }
        }?;

        let text = message.into_text()?;
        let value = serde_json::from_str(&text)?;
        GatewayPayload::from_json(value).map_err(Error::from)
    }

    pub async fn write_payload(
        &mut self,
        payload: GatewayPayload,
    ) -> impl Future<Output = Result<()>> {
        Self::write_to_stream(&mut self.stream, payload)
    }

    async fn write_to_stream(stream: &mut WsStream, payload: GatewayPayload) -> Result<()> {
        let str = serde_json::to_string(&payload.to_json()?)?;
        stream.send(Message::Text(str.into())).await?;

        Ok(())
    }
}
