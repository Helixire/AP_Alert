use futures_util::{stream::FuturesUnordered, SinkExt, StreamExt};

use tokio::{net::TcpStream, task::JoinHandle};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};
use tracing::{debug, error, info};

use crate::ap::messages::Connect;

use super::messages::APMessage;

#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub ip: String,
    pub port: String,
    pub slot: String,
    pub password: String,
}

impl Default for ConnectionInfo {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".to_owned(),
            port: "3828".to_owned(),
            slot: Default::default(),
            password: Default::default(),
        }
    }
}

pub async fn connect(connection_info: ConnectionInfo) -> anyhow::Result<JoinHandle<()>> {
    let stream = match tokio_tungstenite::connect_async(format!(
        "wss://{}:{}",
        connection_info.ip, connection_info.port
    ))
    .await
    {
        Ok((stream, _)) => stream,
        Err(err) => {
            if let tokio_tungstenite::tungstenite::Error::Tls(_) = err {
                let (stream, _) = tokio_tungstenite::connect_async(format!(
                    "ws://{}:{}",
                    connection_info.ip, connection_info.port
                ))
                .await?;
                stream
            } else {
                return Err(err.into());
            }
        }
    };
    info!("Connected");

    Ok(tokio::spawn(handle_ws(connection_info, stream)))
}

async fn on_ap_event(
    connection_info: &ConnectionInfo,
    event: APMessage,
) -> Option<impl Iterator<Item = APMessage>> {
    debug!("{:?}", event);
    match event {
        APMessage::RoomInfo(_) => {
            let message = APMessage::Connect(Connect {
                name: connection_info.slot.clone(),
                password: connection_info.password.clone(),
                ..Default::default()
            });
            return Some([message].into_iter());
        }
        APMessage::Connect(_) => todo!(),
    }
}

async fn handle_ws(
    connection_info: ConnectionInfo,
    connection: WebSocketStream<MaybeTlsStream<TcpStream>>,
) {
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
                                let mut futures = messages
                                    .into_iter()
                                    .map(|m| on_ap_event(&connection_info, m))
                                    .collect::<FuturesUnordered<_>>();
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
