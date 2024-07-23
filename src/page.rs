mod auth;
mod dashboard;

use std::path::PathBuf;

use iced::{executor, Application, Command, Element, Theme};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::ap::connection::{self, connect, ConnectionInfo};
use auth::Auth;
use dashboard::Dashboard;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Context {
    pub connection_info: ConnectionInfo,
    #[serde(skip)]
    pub worker_channel: Option<connection::Connection>,
}

pub struct Page {
    context: Context,
    cur_view: Box<dyn View>,
}

#[derive(Debug, Clone)]
pub enum Pages {
    Connection,
    Dashboard,
}

#[derive(Debug, Clone)]
pub enum Message {
    PseudoInputChanged(String),
    ServerIPInputChanged(String),
    ServerPortInputChanged(String),
    ServerPasswordInputChanged(String),
    Error(String),
    ChangePage(Pages),
    WSEvent(connection::Event),
    Connect,
}

const QUALIFIER: &'static str = "pw";
const ORGANIZATION: &'static str = "olympus_inc";
const APPLICATION: &'static str = "APAlert";
const CONFIG_FILE_NAME: &'static str = "config.json";

fn get_config_path() -> PathBuf {
    let path = directories::ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION).unwrap();

    std::fs::create_dir_all(path.config_dir()).unwrap();
    path.config_dir().join(CONFIG_FILE_NAME)
}

impl Context {
    fn try_load_from_save() -> Self {
        match std::fs::File::open(get_config_path()) {
            Ok(file) => {
                match serde_json::from_reader(file) {
                    Ok(s) => s,
                    Err(_) => Self::default(),
                }
            },
            Err(_) => {
                info!("Could not load save File, using default");
                Self::default()
            },
        }
    }

    fn save(&self) {
        let file = std::fs::File::create(get_config_path()).unwrap();

        serde_json::to_writer_pretty(file, &self).unwrap();
    }
}

pub trait View {
    fn title(&self) -> String;
    fn update(&mut self, message: Message, context: &mut Context) -> Command<Message>;
    fn view(&self, context: &Context) -> Element<Message>;
}

impl Application for Page {
    type Message = Message;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                context: Context::try_load_from_save(),
                cur_view: Box::new(Auth {}),
            },
            Command::none(),
        )
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dracula
    }

    fn title(&self) -> String {
        self.cur_view.title()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ChangePage(page) => {
                match page {
                    Pages::Connection => {
                        self.cur_view = Box::new(Auth {});
                    },
                    Pages::Dashboard => {
                        self.cur_view = Box::new(Dashboard {});
                    },
                }
                Command::none()
            },
            Message::WSEvent(connection::Event::WorkerReady(con)) => {
                self.context.worker_channel.replace(con);

                Command::none()
            },
            _ => self.cur_view.update(message, &mut self.context)
        }
    }

    fn view(&self) -> Element<Message> {
        self.cur_view.view(&self.context)
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        connect().map(Message::WSEvent)
    }

    type Executor = executor::Default;

    type Theme = Theme;

    type Flags = ();
}
