pub struct HTMLParser {
    input: String,
    position: usize,
    current_char: char,
    chars: Vec<char>,
}

impl HTMLParser {
    pub fn new(input: String) -> HTMLParser {
        let chars = input.chars().collect::<Vec<char>>();
        HTMLParser {
            input,
            position: 0,
            chars: chars.clone(),
            current_char: chars[0],
        }
    }

    fn parse_attribute(&mut self) -> (String, String) {
        let name = self.parse_identifier();
        // self.consume_whitespace();
        self.consume_char('=');
        // self.consume_whitespace();
        let value = self.parse_string();
        (name, value)
    }

    fn parse_identifier(&mut self) -> String {
        let mut result = String::new();
        while self.current_char.is_alphanumeric() {
            result.push(self.current_char);
            self.next_char();
        }
        result
    }

    fn parse_string(&mut self) -> String {
        let mut result = String::new();
        self.consume_char('"');
        while self.current_char != '"' {
            result.push(self.current_char);
            self.next_char();
        }
        self.consume_char('"');
        result
    }

    fn consume_char(&mut self, c: char) {
        if self.current_char == c {
            self.next_char();
        } else {
            panic!("Expected char: {}, got: {}", c, self.current_char);
        }
    }

    fn next_char(&mut self) {
        self.position += 1;
        if self.position > self.input.len() - 1 {
            self.current_char = '\0';
        } else {
            self.current_char = self.chars[self.position];
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_attribute() {
        assert_eq!(
            HTMLParser::new("id=\"main\"".to_string()).parse_attribute(),
            (String::from("id"), String::from("main")),
        );

        // many values
        assert_eq!(
            HTMLParser::new("class=\"mt-1 pa-2 text-input\"".to_string()).parse_attribute(),
            (String::from("class"), String::from("mt-1 pa-2 text-input")),
        );

        // includes whitespace
        assert_eq!(
            HTMLParser::new(
                r#"class =
"mt-1""#
                    .to_string()
            )
            .parse_attribute(),
            (String::from("class"), String::from("mt-1")),
        );
    }
}
