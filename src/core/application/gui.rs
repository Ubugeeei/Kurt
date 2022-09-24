use crate::core::http::fetch::fetch_html;
use crate::core::render::render_document;

use gtk::gio::ApplicationFlags;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::Application;

pub fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    // painting
    let app = Application::new(Some("com.example.App"), ApplicationFlags::HANDLES_OPEN);
    app.connect_startup(|_| load_app_style());
    app.connect_activate(build_gui);
    app.run();
    Ok(())
}

fn build_gui(app: &gtk::Application) {
    // create the main window
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("gtk input")
        .width_request(1600)
        .height_request(1000)
        .build();

    let main_container = gtk::Box::new(gtk::Orientation::Vertical, 6);
    window.set_child(Some(&main_container));

    let header_search_bar = gtk::Entry::builder()
        .margin_top(10)
        .margin_start(10)
        .margin_end(20)
        .css_classes(vec!["input".to_string()])
        .build();
    main_container.append(&header_search_bar);
    window.present();

    /*
     *
     * handling on key press enter
     *
     * fetch html from url and render it
     *
     */
    header_search_bar.connect_activate(clone!(@strong main_container => move |header_search_bar| {
        let url = header_search_bar.text().to_string();
        let html = fetch_html(&url);
        println!("---------------------------------------------------------");
        println!("[\x1b[32mFetch HTML: (url: {})\x1b[0m]", url);
        println!("---------------------------------------------------------");
        println!("content");
        println!("\n\x1b[30m{}\n...\x1b[0m\n", &html[..100]);
        render_document(&html, &main_container);
    }));
}

fn load_app_style() {
    // Load the CSS file and add it to the provider
    let provider = gtk::CssProvider::new();
    provider.load_from_data(
        r#"
            .input {
                border-radius: 50px;
                padding-left: 10px;
                padding-right: 10px;
                outline: none;
                font-size: 15px;
                color: #888;
            }

            .kurt-text-default {
                font-size: 15px;
                color: #222;
            }
    "#
        .as_bytes(),
    );

    // Add the provider to the default screen
    gtk::StyleContext::add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
