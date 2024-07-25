use iced::{widget::row, Command};
use iced::widget::{text, Space};

use super::View;

pub struct Dashboard {
}

impl View for Dashboard {
    fn title(&self) -> String {
        String::from("AP_Alert")
    }

    fn update(&mut self, message: super::Message, context: &mut super::Context) -> iced::Command<super::Message> {
        match message {
            _ => Command::none()
        }
    }

    fn view(&self, context: &super::Context) -> iced::Element<super::Message> {
        iced::widget::container::Container::new(
            row![
                text(format!("Slot: {} - Server: {}:{}", context.connection_info.slot, context.connection_info.ip, context.connection_info.port))
                    .horizontal_alignment(iced::alignment::Horizontal::Right),
                Space::with_width(100)

            ]
        )
        .center_y()
        .center_x()
        .into()
    }

}