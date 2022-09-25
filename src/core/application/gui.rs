use std::cell::RefCell;
use std::rc::Rc;

use crate::core::history::Histories;
use crate::core::http::fetch::fetch_html;
use crate::core::render::render_document;

use gtk::gio::ApplicationFlags;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::Application;
use gtk::Button;

pub fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    // painting
    let app = Application::new(Some("com.example.App"), ApplicationFlags::HANDLES_OPEN);
    app.connect_startup(|_| load_app_style());
    app.connect_activate(build_gui);
    app.run();
    Ok(())
}

fn build_gui(app: &gtk::Application) {
    let histories = Rc::new(RefCell::new(Histories::new()));

    /*
     *
     * elements definition
     *
     */
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("gtk input")
        .width_request(1400)
        .height_request(800)
        .build();
    let document_container = gtk::Box::new(gtk::Orientation::Vertical, 6); // html rendering area
    let container = gtk::Box::new(gtk::Orientation::Vertical, 3);
    let header_container = gtk::Box::new(gtk::Orientation::Horizontal, 3);
    let back_document_btn = Button::builder()
        // NOTE: http://standards.freedesktop.org/icon-naming-spec/icon-naming-spec-latest.html
        .icon_name("go-previous")
        .css_classes(vec!["header-nav-btn".to_string()])
        .margin_top(20)
        .margin_bottom(12)
        .margin_start(5)
        .build();
    let forward_document_btn = Button::builder()
        // NOTE: http://standards.freedesktop.org/icon-naming-spec/icon-naming-spec-latest.html
        .icon_name("go-next")
        .css_classes(vec!["header-nav-btn".to_string()])
        .margin_top(20)
        .margin_bottom(12)
        .margin_start(5)
        .build();
    let refresh_btn = Button::builder()
        // NOTE: http://standards.freedesktop.org/icon-naming-spec/icon-naming-spec-latest.html
        .icon_name("view-refresh")
        .css_classes(vec!["header-nav-btn".to_string()])
        .margin_top(20)
        .margin_bottom(12)
        .margin_end(5)
        .build();
    let header_search_bar = gtk::Entry::builder()
        .margin_top(10)
        .margin_start(10)
        .margin_end(20)
        .height_request(10)
        .width_request(1400)
        .css_classes(vec!["input".to_string()])
        .build();
    let body_container = gtk::Box::new(gtk::Orientation::Vertical, 6);
    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .height_request(800)
        .css_classes(vec!["scrolled-window".to_string()])
        .child(&document_container)
        .build();
    let default_text = gtk::Label::builder()
        .label("Type \"localhost:3000/\" and enter to get HTML!")
        .css_classes(vec!["body-default-message".to_string()])
        .build();

    /*
     *
     * layout
     *
     */
    document_container.append(&default_text);
    header_container.append(&back_document_btn);
    header_container.append(&forward_document_btn);
    header_container.append(&refresh_btn);
    header_container.append(&header_search_bar);
    window.set_child(Some(&container));
    container.append(&header_container);
    container.append(&body_container);
    container.append(&scrolled_window);

    window.present();

    /*
     *
     * handling on key press enter
     *
     * fetch html from url and render it
     *
     */
    header_search_bar.connect_activate(
        clone!(@strong histories, @strong scrolled_window, @strong refresh_btn => move |header_search_bar| {
            let url = header_search_bar.text().to_string();
            histories.borrow_mut().set(&url);
            render_by_url(&url, &scrolled_window, &refresh_btn);
        }),
    );
    back_document_btn.connect_clicked(
        clone!(@strong histories, @strong scrolled_window, @strong refresh_btn, @strong header_search_bar => move |_| {
            if let Some(his) = histories.borrow_mut().back() {
                header_search_bar.set_text(&his.url);
                render_by_url(&his.url, &scrolled_window, &refresh_btn);
            }
        }),
    );
    forward_document_btn.connect_clicked(
        clone!(@strong histories, @strong scrolled_window, @strong refresh_btn, @strong header_search_bar => move |_| {
            if let Some(his) = histories.borrow_mut().forward() {
                header_search_bar.set_text(&his.url);
                render_by_url(&his.url, &scrolled_window, &refresh_btn);
            }
        }),
    );
    refresh_btn.connect_clicked(
        clone!(@strong scrolled_window, @strong header_search_bar => move |refresh_btn| {
            let url = header_search_bar.text().to_string();
            render_by_url(&url, &scrolled_window, &refresh_btn);
        }),
    );
}

fn load_app_style() {
    // Load the CSS file and add it to the provider
    let provider = gtk::CssProvider::new();
    provider.load_from_data(
        r#"
            .header-nav-btn {
                border: none;
                box-shadow: none;
                border-radius: 50%;
            }

            .input {
                border-radius: 50px;
                padding-left: 24px;
                padding-right: 10px;
                outline: none;
                font-size: 15px;
                color: #888;
            }

            .scrolled-window {
                background-color: #fff;
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

pub fn render_by_url(url: &str, scrolled_window: &gtk::ScrolledWindow, _refresh_btn: &gtk::Button) {
    // refresh_btn.set_icon_name("window-close");
    // reset document_container
    let document_container = gtk::Box::new(gtk::Orientation::Vertical, 6);
    scrolled_window.set_child(Some(&document_container));
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
    // refresh_btn.set_icon_name("view-refresh"); // FIXME: not working!
}
