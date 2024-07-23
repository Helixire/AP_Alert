use iced::{Application, Settings};
use page::Page;

mod ap;
mod page;

fn main() -> iced::Result {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    
    Page::run(Settings::default())
}