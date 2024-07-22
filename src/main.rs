use iced::{Sandbox, Settings};
use user_interface::Connexion;

mod ap;
mod user_interface;

fn main() -> iced::Result {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    Connexion::run(Settings::default())
}