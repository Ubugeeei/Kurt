#[derive(Debug)]
pub struct Histories {
    stack: Vec<History>,
}
impl Default for Histories {
    fn default() -> Self {
        Self::new()
    }
}
impl Histories {
    pub fn new() -> Histories {
        Histories { stack: vec![] }
    }

    pub fn set(&mut self, url: &str) {
        if self.stack.len() > 0 {
            let last = self.stack.last().unwrap();
            if last.url == url {
                return;
            }
        }
        self.stack.push(History::new(url.to_string()));
        dbg!(&self.stack);
    }
}

#[derive(Debug)]
pub struct History {
    pub url: String,
}
impl History {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}
