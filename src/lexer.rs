use crate::token::Token;

pub struct Lexer {
    pub input: String,
    pub cur_token: Option<char>,
    pub peek_token: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut out = Self {
            input: input,
            cur_token: None,
            peek_token: None,
        };

        out.advance();
        out.advance();

        out
    }

    fn advance(&mut self) {
        self.cur_token = self.peek_token;
        self.peek_token = self.input.chars().next();
        if self.input.len() > 0 {
            self.input = self.input.split_at(1).1.to_string();
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.peek_token
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        use Token::*;

        if let Some(tok) = self.cur_token {
            if tok.is_whitespace() {
                self.skip_whitespace()
            }
        }

        let out = match self.cur_token.ok_or(LexerError::EOF)? {
            '=' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    EQ
                } else {
                    ASSIGN
                }
            }
            '!' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    NOT_EQ
                } else {
                    BANG
                }
            }
            '-' => MINUS,
            '/' => SLASH,
            '+' => PLUS,
            '*' => ASTERISK,
            ',' => COMMA,
            ';' => SEMICOLON,
            '<' => LT,
            '>' => GT,
            '(' => LPAREN,
            ')' => RPAREN,
            '{' => LBRACE,
            '}' => RBRACE,
            c if is_letter(c) => return Ok(self.read_ident()),
            c if c.is_numeric() => return Ok(self.read_int()),
            c => Err(LexerError::Illegal(c))?,
        };

        self.advance();

        Ok(out)
    }

    fn read_while(&mut self, condition: fn(char) -> bool) -> String {
        let mut out_str = String::new();

        while let Some(tok) = self.cur_token {
            if !condition(tok) {
                break;
            }

            out_str.push(tok);
            self.advance();
        }

        out_str
    }

    fn read_ident(&mut self) -> Token {
        Token::ident_from_string(self.read_while(is_letter))
    }

    fn read_int(&mut self) -> Token {
        Token::INT(self.read_while(char::is_numeric))
    }

    fn skip_whitespace(&mut self) {
        _ = self.read_while(char::is_whitespace)
    }
}

fn is_letter(val: char) -> bool {
    val.is_alphabetic() || val == '_'
}

#[derive(Debug, PartialEq)]
pub enum LexerError {
    Illegal(char),
    EOF,
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::token::Token;

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        10 == 10;
        10 != 9;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }"#;

        let expected = [
            Ok(Token::LET),
            Ok(Token::IDENT("five".to_string())),
            Ok(Token::ASSIGN),
            Ok(Token::INT("5".to_string())),
            Ok(Token::SEMICOLON),
            Ok(Token::LET),
            Ok(Token::IDENT("ten".to_string())),
            Ok(Token::ASSIGN),
            Ok(Token::INT("10".to_string())),
            Ok(Token::SEMICOLON),
            Ok(Token::LET),
            Ok(Token::IDENT("add".to_string())),
            Ok(Token::ASSIGN),
            Ok(Token::FUNCTION),
            Ok(Token::LPAREN),
            Ok(Token::IDENT("x".to_string())),
            Ok(Token::COMMA),
            Ok(Token::IDENT("y".to_string())),
            Ok(Token::RPAREN),
            Ok(Token::LBRACE),
            Ok(Token::IDENT("x".to_string())),
            Ok(Token::PLUS),
            Ok(Token::IDENT("y".to_string())),
            Ok(Token::SEMICOLON),
            Ok(Token::RBRACE),
            Ok(Token::SEMICOLON),
            Ok(Token::LET),
            Ok(Token::IDENT("result".to_string())),
            Ok(Token::ASSIGN),
            Ok(Token::IDENT("add".to_string())),
            Ok(Token::LPAREN),
            Ok(Token::IDENT("five".to_string())),
            Ok(Token::COMMA),
            Ok(Token::IDENT("ten".to_string())),
            Ok(Token::RPAREN),
            Ok(Token::SEMICOLON),
            Ok(Token::BANG),
            Ok(Token::MINUS),
            Ok(Token::SLASH),
            Ok(Token::ASTERISK),
            Ok(Token::INT("5".to_string())),
            Ok(Token::SEMICOLON),
            Ok(Token::INT("5".to_string())),
            Ok(Token::LT),
            Ok(Token::INT("10".to_string())),
            Ok(Token::GT),
            Ok(Token::INT("5".to_string())),
            Ok(Token::SEMICOLON),
            Ok(Token::INT("10".to_string())),
            Ok(Token::EQ),
            Ok(Token::INT("10".to_string())),
            Ok(Token::SEMICOLON),
            Ok(Token::INT("10".to_string())),
            Ok(Token::NOT_EQ),
            Ok(Token::INT("9".to_string())),
            Ok(Token::SEMICOLON),
            Ok(Token::IF),
            Ok(Token::LPAREN),
            Ok(Token::INT("5".to_string())),
            Ok(Token::LT),
            Ok(Token::INT("10".to_string())),
            Ok(Token::RPAREN),
            Ok(Token::LBRACE),
            Ok(Token::RETURN),
            Ok(Token::TRUE),
            Ok(Token::SEMICOLON),
            Ok(Token::RBRACE),
            Ok(Token::ELSE),
            Ok(Token::LBRACE),
            Ok(Token::RETURN),
            Ok(Token::FALSE),
            Ok(Token::SEMICOLON),
            Ok(Token::RBRACE),
            Err(LexerError::EOF),
        ];

        let mut lexer = Lexer::new(input.to_string());

        for (i, tt) in expected.iter().enumerate() {
            let token = lexer.next_token();

            if token != *tt {
                println!("Failed: [{i}], expected: {:?}, got: {:?}", tt, token);
                assert!(false)
            }
        }
    }
}
