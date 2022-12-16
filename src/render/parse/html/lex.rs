use super::token::HTMLToken;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    pub fn lex(&mut self) -> Vec<HTMLToken> {
        let mut tokens = Vec::new();
        let mut token = HTMLToken::Lt;
        while token != HTMLToken::EOF {
            token = match self.ch {
                '<' => HTMLToken::Lt,
                '>' => HTMLToken::Gt,
                '=' => HTMLToken::Eq,
                '"' => HTMLToken::DoubleQuot,
                '\'' => HTMLToken::SingleQuot,
                '!' => HTMLToken::Exclamation,
                '/' => HTMLToken::Slash,
                '\t' => HTMLToken::Tab,
                '\n' => HTMLToken::Newline,
                '\x20' => HTMLToken::Space,
                '\0' => HTMLToken::EOF,
                _ => {
                    if self.is_letter() {
                        let ident = self.read_words();
                        HTMLToken::Word(ident)
                    } else {
                        panic!()
                    }
                }
            };
            tokens.push(token.clone());
            self.read_char();
        }
        tokens
    }

    fn read_words(&mut self) -> String {
        let position = self.position;
        while self.is_letter() {
            self.read_char();
        }
        let w = self.input[position..self.position].to_string();
        self.back_char();
        w
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn back_char(&mut self) {
        self.read_position -= 1;
        self.position = self.read_position;
        self.ch = self.input.chars().nth(self.position).unwrap();
    }

    fn is_letter(&self) -> bool {
        self.ch != '\0'
            && self.ch != '\x20'
            && self.ch != '\t'
            && self.ch != '\n'
            && self.ch != '<'
            && self.ch != '>'
            && self.ch != '='
            && self.ch != '"'
            && self.ch != '\''
            && self.ch != '!'
            && self.ch != '/'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let input = String::from(
            r#"<!DOCTYPE html>
<html lang="en">
<body>
    <p class="mr-1 ml-4">Hello, world!</p>
</body>
</html>
"#,
        );
        let mut l = Lexer::new(input);
        let tokens = l.lex();
        assert_eq!(
            tokens,
            vec![
                HTMLToken::Lt,
                HTMLToken::Exclamation,
                HTMLToken::Word(String::from("DOCTYPE")),
                HTMLToken::Space,
                HTMLToken::Word(String::from("html")),
                HTMLToken::Gt,
                HTMLToken::Newline,
                HTMLToken::Lt,
                HTMLToken::Word(String::from("html")),
                HTMLToken::Space,
                HTMLToken::Word(String::from("lang")),
                HTMLToken::Eq,
                HTMLToken::DoubleQuot,
                HTMLToken::Word(String::from("en")),
                HTMLToken::DoubleQuot,
                HTMLToken::Gt,
                HTMLToken::Newline,
                HTMLToken::Lt,
                HTMLToken::Word(String::from("body")),
                HTMLToken::Gt,
                HTMLToken::Newline,
                HTMLToken::Space,
                HTMLToken::Space,
                HTMLToken::Space,
                HTMLToken::Space,
                HTMLToken::Lt,
                HTMLToken::Word(String::from("p")),
                HTMLToken::Space,
                HTMLToken::Word(String::from("class")),
                HTMLToken::Eq,
                HTMLToken::DoubleQuot,
                HTMLToken::Word(String::from("mr-1")),
                HTMLToken::Space,
                HTMLToken::Word(String::from("ml-4")),
                HTMLToken::DoubleQuot,
                HTMLToken::Gt,
                HTMLToken::Word(String::from("Hello,")),
                HTMLToken::Space,
                HTMLToken::Word(String::from("world")),
                HTMLToken::Exclamation,
                HTMLToken::Lt,
                HTMLToken::Slash,
                HTMLToken::Word(String::from("p")),
                HTMLToken::Gt,
                HTMLToken::Newline,
                HTMLToken::Lt,
                HTMLToken::Slash,
                HTMLToken::Word(String::from("body")),
                HTMLToken::Gt,
                HTMLToken::Newline,
                HTMLToken::Lt,
                HTMLToken::Slash,
                HTMLToken::Word(String::from("html")),
                HTMLToken::Gt,
                HTMLToken::Newline,
                HTMLToken::EOF,
            ]
        );
    }
}
