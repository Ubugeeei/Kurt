mod core;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    core::application::gui::start_app()
}
