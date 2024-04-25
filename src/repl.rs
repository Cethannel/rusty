use rusty::lexer::Lexer;

use std::{io::Read, io::Write};

const PROMPT: &str = ">>";

pub fn start<R: Read, W: Write>(mut in_rd: R, mut out: W) {
    loop {
        write!(out, "{}", PROMPT).unwrap();
        out.flush().unwrap();
        let mut line = String::new();
        loop {
            let mut buf = [0; 1];
            in_rd.read(&mut buf).unwrap();
            let c = buf[0] as char;
            if c == '\n' {
                break;
            }
            line.push(c);
        }
        let mut lexer = Lexer::new(line);

        let mut tok = lexer.next_token();

        while tok != Err(rusty::lexer::LexerError::EOF) {
            writeln!(&mut out, "{:?}", tok).unwrap();
            tok = lexer.next_token();
        }
    }
}
