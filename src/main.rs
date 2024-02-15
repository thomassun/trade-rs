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
    let case_url = Url::parse("wss://stream.binance.com:443/ws").expect("BAD URL");
    let (ws_stream, _) = connect_async(case_url).await.map_err(|e| match e {
        tokio_tungstenite::tungstenite::Error::Http(ref res) => {
            if let Some(body) = res.body() {
                println!("{}", String::from_utf8(body.clone()).expect("BADBAD"));
            }
            e
        }
        _ => e,
    })?;
    let (mut writer, mut reader) = ws_stream.split();
    let subscribe = r#"{
    "method":"SUBSCRIBE",
    "params":
        [
            "btcusdt@aggTrade",
            "btcusdt@depth",
            "btcusdt@avgPrice",
            "ethusdt@avgPrice"
        ],
    "id": 1234
    }"#;
    writer.send(Message::Text(subscribe.into())).await?;
    while let Some(msg) = reader.next().await {
        let msg = msg?;
        match msg {
            Message::Ping(payload) => {
                writer.send(Message::Pong(payload)).await?;
                println!("made a PING/PONG");
            }
            Message::Text(text) => {
                print!("{text}");
            }
            Message::Binary(bin) => {
                println!("Unexpected binary received");
                println!("{bin:?}");
            }
            msg => {
                println!("MSG received:{msg:#?}");
            }
        }
    }
    Ok(())
}
