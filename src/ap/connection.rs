use futures_util::{select, SinkExt, StreamExt};

use iced::{futures::channel::mpsc, subscription};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};
use tracing::{error, info};

use crate::ap::messages::Connect;

use super::messages::APMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
            port: "38281".to_owned(),
            slot: Default::default(),
            password: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    WorkerReady(Connection),
    APMessage(super::messages::APMessage),
}

#[derive(Debug, Clone)]
pub struct Connection(pub mpsc::Sender<InputMessage>);

impl Connection {
    pub fn send(&mut self, message: InputMessage) {
        self.0
            .try_send(message)
            .expect("Send message to echo server");
    }
}

pub enum InputMessage {
    Connect(ConnectionInfo),
}

enum State {
    Disconnected,
    Connected(WebSocketStream<MaybeTlsStream<TcpStream>>),
}

pub fn connect() -> iced::Subscription<Event> {
    struct WS;

    subscription::channel(std::any::TypeId::of::<WS>(), 100, |mut output| async move {
        let mut state = State::Disconnected;
        let mut connection_info = None;

        let (sender, mut receiver) = mpsc::channel(100);

        let _ = output.send(Event::WorkerReady(Connection(sender))).await;

        loop {
            match &mut state {
                State::Disconnected => {
                    if let Some(connection_info) = &connection_info {
                        match connect_to_ws(connection_info).await {
                            Err(err) => {
                                error!("{}", err);
                            }
                            Ok(server) => {
                                state = State::Connected(server);
                            }
                        }
                    }

                    match receiver.select_next_some().await {
                        InputMessage::Connect(info) => connection_info = Some(info),
                    };
                }
                State::Connected(server) => {
                    let mut fused_websocket = server.by_ref().fuse();

                    select! {
                        message = fused_websocket.select_next_some() => {
                            match message {
                                Ok(Message::Text(t)) => {
                                    match serde_json::from_str::<Vec<APMessage>>(&t) {
                                        Err(err) => error!("Failed converting to APMessage {:?}", err),
                                        Ok(messages) => {
                                            for message in messages {
                                                match message {
                                                    APMessage::RoomInfo(_) => {
                                                        if let Some(info) = &connection_info {
                                                            let message = APMessage::Connect(Connect {
                                                                name: info.slot.clone(),
                                                                password: info.password.clone(),
                                                                ..Default::default()
                                                            });
                                                            if let Err(err) = fused_websocket.send(Message::Text(serde_json::to_string(&[message]).unwrap())).await {
                                                                error!("{}", err);
                                                            }
                                                        }
                                                    },
                                                    _ => {
                                                        let _ = output.send(Event::APMessage(message)).await;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                                Err(_) => state = State::Disconnected,
                                Ok(_) => {},
                            }
                        }

                        gui_event = receiver.select_next_some() => {
                            match gui_event {
                                InputMessage::Connect(info) => {
                                    connection_info.replace(info);
                                    state = State::Disconnected;
                                },
                            }
                        }
                    }
                }
            }
        }
    })
}

async fn connect_to_ws(
    connection_info: &ConnectionInfo,
) -> anyhow::Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
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
    Ok(stream)
}