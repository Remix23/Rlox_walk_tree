use crate::scanner::{Token, TokenType};

pub struct ParseError {
    line : i32,
    message : String,
}

pub struct RuntimeError {
    token : Token,
    message : String,
}

pub struct ScannerError {
    line : i32,
    message : String,
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

pub fn parse_error (token : &Token, msg : &str) -> LoxError{
    if token.token_type == TokenType::EOF {
        report(token.line, " at end".to_string(), msg);
    } else {
        report(token.line, format!(" at '{}'", token.lexeme), msg);
    }

    return LoxError::ParseError({
        ParseError {
            line: token.line,
            message: msg.to_string(),
        }
    });
}