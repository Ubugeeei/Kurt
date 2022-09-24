type HTMLString = String;

pub fn fetch_html(url: &str) -> HTMLString {
    let req_url = if url.len() > 0 {
        if &url[0..4] == "http" {
            url.to_string()
        } else {
            format!("http://{}", url)
        }
    } else {
        "".to_string()
    };
    let resp = match reqwest::blocking::get(req_url) {
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
