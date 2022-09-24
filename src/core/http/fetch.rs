type HTMLString = String;
pub fn fetch_html(_url: &str) -> HTMLString {
    // TODO: fetch resource from url
    std::fs::read_to_string("./example/example.html").unwrap()
}
