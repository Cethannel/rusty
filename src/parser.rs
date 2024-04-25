use crate::{lexer::Lexer, token::Token};

struct Parser {
    lexer: Lexer,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        let mut out = Self {
            lexer,
            cur_token: None,
            peek_token: None,
        };

        out.next_token();
        out.next_token();

        out
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token().ok();
    }
}
