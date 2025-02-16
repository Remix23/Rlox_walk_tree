use crate::scanner::{Token, TokenType};

pub struct ParseError {
    pub line : i32,
    pub message : String,
}

#[derive(Debug)]
pub struct RuntimeError {
    pub token : Token,
    pub message : String,
}

pub struct ScannerError {
    pub line : i32,
    pub message : String,
}

pub struct ResolverError {
    pub line : i32,
    pub message : String,
}

pub enum LoxError {
    ParseError (ParseError),
    RuntimeError (RuntimeError),
    ScannerError (ScannerError),
}

fn report (line : i32, loc : String, msg : &str) {
    println!("[line {}] Error {}: {}", line, loc, msg);
}

pub fn err (line : i32, msg : &str) -> bool {
    report(line,  "".to_string(), msg);
    // TODO: Rewrite the 
    return true;
}  

pub fn parse_error (token : &Token, msg : &str) -> ParseError{
    if token.token_type == TokenType::EOF {
        report(token.line, " at end".to_string(), msg);
    } else {
        report(token.line, format!(" at '{}'", token.lexeme), msg);
    }

    ParseError {
            line: token.line,
            message: msg.to_string(),
        }
}