#[derive(Debug)]
pub struct Histories {
    cursor: isize,
    stack: Vec<History>,
}
impl Default for Histories {
    fn default() -> Self {
        Self::new()
    }
}
impl Histories {
    pub fn new() -> Histories {
        Histories {
            stack: vec![History {
                url: "".to_string(),
            }],
            cursor: -1,
        }
    }

    pub fn set(&mut self, url: &str) {
        if self.stack.len() > 0 {
            let last = self.stack.last().unwrap();
            if last.url == url {
                return;
            }
        }
        self.stack.push(History::new(url.to_string()));
        self.cursor = self.stack.len() as isize - 1;
        dbg!(&self.stack);
    }

    pub fn forward(&mut self) -> Option<&History> {
        if self.cursor < self.stack.len() as isize - 1 {
            self.cursor += 1;
            return self.stack.get(self.cursor as usize);
        }
        None
    }

    pub fn back(&mut self) -> Option<&History> {
        if self.cursor > 0 {
            self.cursor -= 1;
            return self.stack.get((self.cursor) as usize);
        }
        None
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
