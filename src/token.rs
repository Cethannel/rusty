#[derive(PartialEq, Debug)]
pub enum Token {
    IDENT(String),
    INT(String),

    ASSIGN,
    PLUS,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
}

impl Token {
    pub fn ident_from_string(input: String) -> Token {
        use Token::*;
        match input.as_str() {
            "let" => LET,
            "fn" => FUNCTION,
            _ => Token::IDENT(input),
        }
    }
}
