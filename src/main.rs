use iced::{Application, Settings};
use user_interface::Context;

mod ap;
mod user_interface;

fn main() -> iced::Result {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    Context::run(Settings::default())
}