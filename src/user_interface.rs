use iced::widget::{button, column};
use iced::widget::text_input;
use iced::{Alignment, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Connexion::run(Settings::default())
}

struct Connexion {
    pseudo: String,
    server_ip: String,
    server_port: String,
    password: String
}

#[derive(Debug, Clone)]
enum Message {
    PseudoInputChanged(String),
    ServerIPInputChanged(String),
    ServerPortInputChanged(String),
    ServerPasswordInputChanged(String),
    Connect,
}

impl Sandbox for Connexion {
    type Message = Message;

    fn new() -> Self {
        Self {
            pseudo: "".to_owned(),
            server_ip: "".to_owned(),
            server_port: "38281".to_owned(),
            password: "".to_owned(),
        }
    }

    fn title(&self) -> String {
        String::from("AP_Alert")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Connect => {
                println!("attempting connexion");
                // todo : send info to connexion Module
            },
            Message::PseudoInputChanged(updated_pseudo) => {
                self.pseudo = updated_pseudo;
            }
            Message::ServerIPInputChanged(updated_server_ip) => {
                self.server_ip = updated_server_ip;
            }
            Message::ServerPortInputChanged(updated_port) => {
                self.server_port = updated_port;
            }
            Message::ServerPasswordInputChanged(updated_password) => {
                self.password = updated_password;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            text_input("Pseudo", &self.pseudo).on_input(Message::PseudoInputChanged),
            text_input("Server IP", &self.server_ip).on_input(Message::ServerIPInputChanged),
            text_input("Server port", &self.server_port).on_input(Message::ServerPortInputChanged),
            text_input("Server password", &self.password).on_input(Message::ServerPasswordInputChanged),
            button("Connect").on_press(Message::Connect)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
