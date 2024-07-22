
use std::sync::{Arc, RwLock};

use iced::widget::{button, column, row, text};
use iced::widget::text_input;
use iced::{executor, Alignment, Application, Command, Element, Theme};
use tokio::task::JoinHandle;
use tracing::{error, info};

use crate::ap::connection::{connect, ConnectionInfo};

pub struct Context {
    connection_info: ConnectionInfo,
    connection_handle: Arc<RwLock<Option<JoinHandle<()>>>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    PseudoInputChanged(String),
    ServerIPInputChanged(String),
    ServerPortInputChanged(String),
    ServerPasswordInputChanged(String),
    Connect,
}

impl Application for Context {
    type Message = Message;

    
    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self {
            connection_info: ConnectionInfo::default(),
            connection_handle: Arc::new(RwLock::new(None)),
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("AP_Alert")
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dracula
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Connect => {
                info!("attempting connexion");
                let write_connection = self.connection_handle.clone();
                Command::perform(connect(self.connection_info.clone()), move |ret|  {
                    match ret {
                        Ok(join) => {
                            write_connection.write().unwrap().replace(join);
                        },
                        Err(err) => error!("Could not connect: {:?}", err), // TODO Write message for user to see 
                    };
                    Message::PseudoInputChanged("POPO".to_owned()) // TODO Change state of client to 
                })
            },
            Message::PseudoInputChanged(updated_pseudo) => {
                self.connection_info.slot = updated_pseudo;
                
                Command::none()
            }
            Message::ServerIPInputChanged(updated_server_ip) => {
                self.connection_info.ip = updated_server_ip;
                
                Command::none()
            }
            Message::ServerPortInputChanged(updated_port) => {
                self.connection_info.port = updated_port;
                
                Command::none()
            }
            Message::ServerPasswordInputChanged(updated_password) => {
                self.connection_info.password = updated_password;

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![text("Pseudo: "), text_input("Pseudo", &self.connection_info.slot).on_input(Message::PseudoInputChanged)],
            row![text("Server IP: "), text_input("Server IP", &self.connection_info.ip).on_input(Message::ServerIPInputChanged)],
            row![text("Server port: "), text_input("Server port", &self.connection_info.port).on_input(Message::ServerPortInputChanged)],
            row![text("Server password: "), text_input("Server password", &self.connection_info.password).on_input(Message::ServerPasswordInputChanged)],
            button("Connect").on_press(Message::Connect)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
    
    type Executor = executor::Default;
    
    type Theme = Theme;
    
    type Flags = ();
}
