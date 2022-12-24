pub struct HTMLParser {
    input: String,
    position: usize,
}

impl HTMLParser {
    pub fn new(input: String) -> HTMLParser {
        HTMLParser { input, position: 0 }
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
