use iced::{Alignment, Command, Element};
use iced::widget::{button, column, row, text, text_input};
use tracing::{info, error};

use crate::ap::connection::connect;

use super::{Context, View, Message};

pub struct Auth {
}

impl View for Auth {
    
    fn view(&self, context: &Context) -> Element<Message> {
        let column = column![
            row![text("Pseudo: "), text_input("Pseudo", &context.connection_info.slot).on_input(Message::PseudoInputChanged)],
            row![text("Server IP: "), text_input("Server IP", &context.connection_info.ip).on_input(Message::ServerIPInputChanged)],
            row![text("Server port: "), text_input("Server port", &context.connection_info.port).on_input(Message::ServerPortInputChanged)],
            row![text("Server password: "), text_input("Server password", &context.connection_info.password).on_input(Message::ServerPasswordInputChanged)],
            button("Connect").on_press(Message::Connect)
        ];
        column
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
    
    fn title(&self) -> String {
        String::from("AP_Alert")
    }
    
    fn update(&mut self, message: super::Message, context: &mut Context) -> iced::Command<super::Message> {
        match message {
            Message::PseudoInputChanged(updated_pseudo) => {
                context.connection_info.slot = updated_pseudo;

                Command::none()
            },
            Message::ServerIPInputChanged(updated_server_ip) => {
                context.connection_info.ip = updated_server_ip;

                Command::none()
            },
            Message::ServerPortInputChanged(updated_port) => {
                context.connection_info.port = updated_port;

                Command::none()
            },
            Message::ServerPasswordInputChanged(updated_password) => {
                context.connection_info.password = updated_password;
                
                Command::none()
            },

            Message::Connect => {
                info!("attempting connexion");
                Command::perform(connect(context.connection_info.clone()), move |ret|  {
                    match ret {
                        Ok(join) => {
                            Message::Connected
                        },
                        Err(err) => Message::Error(err.to_string()) 
                    }
                })
            }

            Message::Error(err) => {
                error!("{}", err);
                
                Command::none()
            },
            Message::Connected => {
                context.save();

                Command::none()
            },
        }
    }
}
