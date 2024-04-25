#[derive(PartialEq, Debug, Clone)]
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

    pub fn literal(&self) -> String {
        use Token::*;
        match self {
            IDENT(ident) => &ident,
            INT(ident) => &ident,
            ASSIGN => "=",
            PLUS => "+",
            BANG => "!",
            MINUS => "-",
            SLASH => "/",
            LT => "<",
            GT => ">",
            ASTERISK => "*",

            EQ => "==",
            NOT_EQ => "!=",

            COMMA => ",",
            SEMICOLON => ";",

            LPAREN => "(",
            RPAREN => ")",
            LBRACE => "{",
            RBRACE => "}",

            FUNCTION => "fn",
            LET => "let",
            IF => "if",
            ELSE => "else",
            RETURN => "return",
            TRUE => "true",
            FALSE => "false",
        }
        .to_string()
    }
}
