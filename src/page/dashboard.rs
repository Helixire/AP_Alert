use iced::{Application, Theme, executor};

struct Dashboard {

}

#[derive(Debug)]
enum Message {
    Reload,
    Exit,
}

impl Application for Dashboard {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        todo!()
    }

    fn title(&self) -> String {
        todo!()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        todo!()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        todo!()
    }
}