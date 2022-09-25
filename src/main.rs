mod application;
mod history;
mod http;
mod javascript;
mod render;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    application::start_app()
}
