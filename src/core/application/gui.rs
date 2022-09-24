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
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("gtk input")
        .width_request(1280)
        .height_request(720)
        .build();

    /*
     *
     * html rendering area
     *
     */
    let document_container = gtk::Box::new(gtk::Orientation::Vertical, 6);
    // default
    let text = gtk::Label::builder()
        .label("Type \"localhost:3000/\" and enter to get HTML!")
        .css_classes(vec!["body-default-message".to_string()])
        .build();
    document_container.append(&text);

    /*
     *
     * base browser areas
     *
     */
    let header_container = gtk::Box::new(gtk::Orientation::Vertical, 3);
    let header_search_bar = gtk::Entry::builder()
        .margin_top(10)
        .margin_start(10)
        .margin_end(20)
        .height_request(10)
        .css_classes(vec!["input".to_string()])
        .build();
    header_container.append(&header_search_bar);
    window.set_child(Some(&header_container));

    let body_container = gtk::Box::new(gtk::Orientation::Vertical, 6);
    header_container.append(&body_container);

    // header_container.append(&document_container);
    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .height_request(720)
        .css_classes(vec!["scrolled-window".to_string()])
        .child(&document_container)
        .build();

    header_container.append(&scrolled_window);

    window.present();

    /*
     *
     * handling on key press enter
     *
     * fetch html from url and render it
     *
     */
    header_search_bar.connect_activate(
        clone!(@strong scrolled_window => move |header_search_bar| {
            // reset document_container
            let document_container = gtk::Box::new(gtk::Orientation::Vertical, 6);
            scrolled_window.set_child(Some(&document_container));

            let url = header_search_bar.text().to_string();
            let html = fetch_html(&url);
            println!("---------------------------------------------------------");
            println!("[\x1b[32mFetch HTML: (url: {})\x1b[0m]", url);
            println!("---------------------------------------------------------");
            println!("content");
            if html.len() > 100 {
                println!("\n\x1b[30m{}\n...\x1b[0m\n", &html[..100]);
            } else {
                println!("{}", html);
            }
            render_document(&html, &document_container);
        }),
    );
}

fn load_app_style() {
    // Load the CSS file and add it to the provider
    let provider = gtk::CssProvider::new();
    provider.load_from_data(
        r#"
            .scrolled-window {
                background-color: #fff;
            }

            .input {
                border-radius: 50px;
                padding-left: 24px;
                padding-right: 10px;
                outline: none;
                font-size: 15px;
                color: #888;
            }

            .body-default-message {
                margin-top: 20px;
                font-size: 20px;
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
