type HTMLString = String;

pub fn fetch_html(url: &str) -> HTMLString {
    let resp = match reqwest::blocking::get(url) {
        Ok(resp) => resp.text(),
        Err(err) => {
            println!("Error: {}", err);
            Err(err)
        }
    };

    match resp {
        Ok(html) => html,
        Err(err) => {
            println!("Error: {}", err);
            "<html><body><p>404 Not Found</p></body></html>".to_string()
        }
    }
}
