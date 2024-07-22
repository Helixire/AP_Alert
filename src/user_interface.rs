
use std::sync::{Arc, RwLock};

use iced::widget::{button, column, row, text};
use iced::widget::text_input;
use iced::{executor, Alignment, Application, Command, Element, Theme};
use tokio::task::JoinHandle;
use tracing::{error, info};

use crate::ap::connection::connect;

pub struct Context {
    pseudo: String,
    server_ip: String,
    server_port: String,
    password: String,
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
            pseudo: "".to_owned(),
            server_ip: "".to_owned(),
            server_port: "38281".to_owned(),
            password: "".to_owned(),
            connection_handle: Arc::new(RwLock::new(None)),
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("AP_Alert")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Connect => {
                info!("attempting connexion");
                let write_connection = self.connection_handle.clone();
                Command::perform(connect(self.server_ip.clone(), self.server_port.clone()), move |ret|  {
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
                self.pseudo = updated_pseudo;
                
                Command::none()
            }
            Message::ServerIPInputChanged(updated_server_ip) => {
                self.server_ip = updated_server_ip;
                
                Command::none()
            }
            Message::ServerPortInputChanged(updated_port) => {
                self.server_port = updated_port;
                
                Command::none()
            }
            Message::ServerPasswordInputChanged(updated_password) => {
                self.password = updated_password;

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![text("Pseudo: "), text_input("Pseudo", &self.pseudo).on_input(Message::PseudoInputChanged)],
            row![text("Server IP: "), text_input("Server IP", &self.server_ip).on_input(Message::ServerIPInputChanged)],
            row![text("Server port: "), text_input("Server port", &self.server_port).on_input(Message::ServerPortInputChanged)],
            row![text("Server password: "), text_input("Server password", &self.password).on_input(Message::ServerPasswordInputChanged)],
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
