use futures_util::SinkExt;
use futures_util::StreamExt;
use reqwest::Url;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::Result;
// use url::Url;
// use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    // let case_url = Url::parse("wss://data-stream.binance.vision/ws").expect("BAD URL");
    let case_url = Url::parse("wss://stream.binance.com:9443").expect("BAD URL");
    let (ws_stream, _) = connect_async(case_url).await?;
    let (mut writer, mut reader) = ws_stream.split();
    let subscribe = r#"{
    "method":"SUBSCRIBE",
    "params":
        [
            "btcusdt@aggTrade",
            "btcusdt@depth",
            "ethusdt@avgPrice"
        ],
    "id": 1234
    }"#;
    writer.send(Message::Text(subscribe.into())).await?;
    while let Some(msg) = reader.next().await {
        let msg = msg?;
        match msg {
            Message::Ping(payload) => {
                println!("PING received");
                writer.send(Message::Pong(payload)).await?;
                println!("PONG sent");
            }
            Message::Text(text) => {
                println!("Text received");
                println!("{text}");
            }
            Message::Binary(bin) => {
                println!("Binary received");
                println!("{bin:?}");
            }
            msg => {
                println!("MSG received:{msg:#?}");
            }
        }
    }
    Ok(())
}
