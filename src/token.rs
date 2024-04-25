#[derive(PartialEq, Debug)]
pub enum Token {
    IDENT(String),
    INT(String),

    ASSIGN,
    PLUS,
    BANG,
    MINUS,
    SLASH,
    LT,
    GT,
    ASTERISK,

    EQ,
    NOT_EQ,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
    IF,
    ELSE,
    RETURN,
    TRUE,
    FALSE,
}

impl Token {
    pub fn ident_from_string(input: String) -> Token {
        use Token::*;
        match input.as_str() {
            "let" => LET,
            "fn" => FUNCTION,
            "if" => IF,
            "else" => ELSE,
            "true" => TRUE,
            "false" => FALSE,
            "return" => RETURN,
            _ => Token::IDENT(input),
        }
    }
}
