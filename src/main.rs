mod ap_messages;

use std::fmt::Display;

use ap_messages::{APMessage, Connect};
use futures_util::{
    stream::FuturesUnordered,
    SinkExt, StreamExt,
};
use tokio::{net::TcpStream, task::JoinHandle};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};
use tracing::{debug, error, info};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let rec = connect("127.0.0.1", 3828).await?;

    rec.await?;

    info!("Bye Bye");
    Ok(())
}

async fn connect(dest: impl Display, port: u32) -> anyhow::Result<JoinHandle<()>> {
    let stream = match tokio_tungstenite::connect_async(format!("wss://{}:{}", dest, port)).await {
        Ok((stream, _)) => stream,
        Err(err) => {
                if let tokio_tungstenite::tungstenite::Error::Tls(_) = err {
                    let (stream, _) = tokio_tungstenite::connect_async(format!("ws://{}:{}", dest, port)).await?;
                    stream
                } else {
                    return Err(err.into());
                }
        },
    };
    info!("Connected");

    Ok(tokio::spawn(handle_ws(stream)))
}

async fn on_ap_event(
    event: APMessage,
) -> Option<impl Iterator<Item = APMessage>> {
    debug!("{:?}", event);
    match event {
        APMessage::RoomInfo(_) => {
            let message = APMessage::Connect(Connect {
                name: "Raptor".to_owned(),
                ..Default::default()
            });
            return Some([message].into_iter());
        }
        APMessage::Connect(_) => todo!(),
    }
}

async fn handle_ws(connection: WebSocketStream<MaybeTlsStream<TcpStream>>) {
    let (mut write, mut read) = connection.split();
    loop {
        match read.next().await {
            None => {
                info!("Connection closed !!");
                break;
            }
            Some(res) => match res {
                Err(err) => error!("Error : {}", err),
                Ok(mes) => match mes {
                    Message::Text(text) => {
                        debug!("Recived Message : {}", text);
                        match serde_json::from_str::<Vec<APMessage>>(&text) {
                            Err(err) => error!("Failed converting to APMessage {:?}", err),
                            Ok(messages) => {
                                let mut futures = messages.into_iter().map(|m|on_ap_event(m)).collect::<FuturesUnordered<_>>();
                                let mut messages = vec![];

                                while let Some(ret) = futures.next().await {
                                    if let Some(m) = ret {
                                        messages.extend(m);
                                    }
                                }
                                if !messages.is_empty() {
                                    let text = serde_json::to_string(&messages).unwrap();
                                    debug!("{}", serde_json::to_string_pretty(&messages).unwrap());
                                    write.send(Message::Text(text)).await.unwrap();
                                }
                            }
                        }
                    }
                    Message::Close(_) => break,
                    _ => {}
                },
            },
        }
    }
}
