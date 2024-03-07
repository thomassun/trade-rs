// use futures_util::FutureExt;
use futures_util::SinkExt;
use futures_util::StreamExt;
use reqwest::Url;
use tokio::select;
use tokio_tungstenite::{
    connect_async_with_config,
    tungstenite::{protocol::WebSocketConfig, Message, Result},
};

async fn fun1() -> Option<usize> {
    0.into()
}
async fn fun2() -> usize {
    0
}
#[tokio::main]
async fn main() -> Result<()> {
    let _c = select! {
       Some(a) = fun1() => a,
       a = fun2() => a,
    };
    // let case_url = Url::parse("wss://data-stream.binance.vision/ws").expect("BAD URL");
    let binance_stream_url = Url::parse("wss://stream.binance.com:443/ws").expect("BAD URL");
    let (ws_stream, _) = connect_async_with_config(
        binance_stream_url,
        Some(WebSocketConfig {
            ..Default::default()
        }),
        false,
    )
    .await
    .map_err(|e| match e {
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
                println!("{text}");
            }
            Message::Binary(bin) => {
                println!("Unexpected binary received");
                println!("{bin:?}");
            }
            msg => {
                println!("other MSG received:{msg:#?}");
            }
        }
    }
    Ok(())
}
