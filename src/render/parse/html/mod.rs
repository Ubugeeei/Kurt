mod lex;
mod token;

pub struct Parser {
    lexer: lex::Lexer,
    tokens: Vec<token::HTMLToken>,
    position: usize,
    read_position: usize,
}

impl Parser {
    fn new(input: String) -> Parser {
        let mut lexer = lex::Lexer::new(input);
        let tokens = lexer.lex();
        Parser {
            lexer,
            tokens,
            position: 0,
            read_position: 0,
        }
    }

    fn next(&mut self) -> token::HTMLToken {
        let token = self.tokens[self.position].clone();
        self.position += 1;
        token
    }

    fn peek(&self, idx: usize) -> token::HTMLToken {
        self.tokens[self.position + idx].clone()
    }

    fn peek_non_white_token(&self) -> token::HTMLToken {
        let mut idx = 0;
        loop {
            let token = self.tokens[self.position + idx].clone();
            if token != token::HTMLToken::Space
                && token != token::HTMLToken::Tab
                && token != token::HTMLToken::Newline
            {
                return token;
            }
            idx += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::render::parse::html::token::HTMLToken;

    use super::*;

    #[test]
    fn test_next() {
        let input = String::from("<!DOCTYPE html>");
        let mut parser = Parser::new(input);
        assert_eq!(parser.next(), HTMLToken::Lt);
        assert_eq!(parser.next(), HTMLToken::Exclamation);
        assert_eq!(parser.next(), HTMLToken::Word(String::from("DOCTYPE")));
        assert_eq!(parser.next(), HTMLToken::Space);
        assert_eq!(parser.next(), HTMLToken::Word(String::from("html")));
        assert_eq!(parser.next(), HTMLToken::Gt);
    }
}
