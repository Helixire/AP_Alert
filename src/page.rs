mod auth;

use std::sync::{Arc, RwLock};

use iced::{executor, Application, Command, Element, Theme};
use tokio::task::JoinHandle;

use crate::ap::connection::ConnectionInfo;
use auth::Auth;

pub struct Context {
    pub connection_info: ConnectionInfo,
    connection_handle: Arc<RwLock<Option<JoinHandle<()>>>>,
}

pub struct Page {
    context: Context,
    cur_view: Box<dyn View>,
}

#[derive(Debug, Clone)]
pub enum Message {
    PseudoInputChanged(String),
    ServerIPInputChanged(String),
    ServerPortInputChanged(String),
    ServerPasswordInputChanged(String),
    Connect,
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
                context: Context {
                    connection_info: ConnectionInfo::default(),
                    connection_handle: Arc::new(RwLock::new(None)),
                },
                cur_view: Box::new(Auth {}),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        self.cur_view.title()
        // String::from("AP_Alert")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        return self.cur_view.update(message, &mut self.context);
    }

    fn view(&self) -> Element<Message> {
        self.cur_view.view(&self.context)
    }

    type Executor = executor::Default;

    type Theme = Theme;

    type Flags = ();
}
