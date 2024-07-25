use iced::widget::{button, column, row, text, text_input, Space};
use iced::{Alignment, Command, Element, Length};
use tracing::{error, info};

use crate::ap::connection;

use super::{Context, Message, Pages, View};

pub struct Auth {}

impl View for Auth {
    fn view(&self, context: &Context) -> Element<Message> {
        iced::widget::container::Container::new(
            column![
                column![
                    row![
                        text("Slot: ")
                            .width(100)
                            .horizontal_alignment(iced::alignment::Horizontal::Right),
                        text_input("Slot", &context.connection_info.slot)
                            .width(300)
                            .on_input(Message::PseudoInputChanged),
                        Space::with_width(100)
                    ]
                    .align_items(Alignment::Center),
                    row![
                        text("IP: ")
                            .width(100)
                            .horizontal_alignment(iced::alignment::Horizontal::Right),
                        text_input("IP", &context.connection_info.ip)
                            .width(300)
                            .on_input(Message::ServerIPInputChanged),
                        Space::with_width(100)
                    ]
                    .align_items(Alignment::Center),
                    row![
                        text("Port: ")
                            .width(100)
                            .horizontal_alignment(iced::alignment::Horizontal::Right),
                        text_input("Port", &context.connection_info.port)
                            .width(300)
                            .on_input(Message::ServerPortInputChanged),
                        Space::with_width(100)
                    ]
                    .align_items(Alignment::Center),
                    row![
                        text("Password: ")
                            .width(100)
                            .horizontal_alignment(iced::alignment::Horizontal::Right),
                        text_input("Password", &context.connection_info.password)
                            .width(300)
                            .on_input(Message::ServerPasswordInputChanged),
                        Space::with_width(100)
                    ]
                    .align_items(Alignment::Center),
                ]
                .spacing(5),
                button("Connect").on_press(Message::Connect)
            ]
            .align_items(Alignment::Center)
            .spacing(20),
        )
        .center_y()
        .center_x()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn title(&self) -> String {
        String::from("AP_Alert")
    }

    fn update(
        &mut self,
        message: super::Message,
        context: &mut Context,
    ) -> iced::Command<super::Message> {
        match message {
            Message::PseudoInputChanged(updated_pseudo) => {
                context.connection_info.slot = updated_pseudo;

                Command::none()
            }
            Message::ServerIPInputChanged(updated_server_ip) => {
                context.connection_info.ip = updated_server_ip;

                Command::none()
            }
            Message::ServerPortInputChanged(updated_port) => {
                context.connection_info.port = updated_port;

                Command::none()
            }
            Message::ServerPasswordInputChanged(updated_password) => {
                context.connection_info.password = updated_password;

                Command::none()
            }

            Message::Connect => {
                info!("attempting connexion");
                if let Some(c) = &mut context.worker_channel {
                    c.send(connection::InputMessage::Connect(context.connection_info.clone()));
                }

                Command::none()
            }

            Message::WSEvent(connection::Event::APMessage(crate::ap::messages::APServerMessage::Connected(_))) => {
                context.save();
                Command::perform(async{}, |_| Message::ChangePage(Pages::Dashboard))
            }

            Message::Error(err) => {
                error!("{}", err);

                Command::none()
            }
            _ => { Command::none() }
        }
    }
}
