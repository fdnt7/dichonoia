use dichonoia::gateway::GatewayClient;
use dichonoia::http::HttpClient;
use dichonoia_models::gateway::GatewayIntents;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let token = std::env::var("BOT_TOKEN")?;
    let http = HttpClient::new(&token);

    let intents = GatewayIntents::GUILDS;
    let mut gateway = GatewayClient::connect(&token, intents).await?;
    println!("Connected to gateway");

    loop {
        let payload = gateway.read_payload().await?;
        println!("Payload: {payload:#?}")
    }

    Ok(())
}
