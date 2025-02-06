use core::num;
use::std::fmt::Display;
use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::error_handler::err;


#[derive(Debug, Clone)]
enum TokenType {
    // Single char
    LeftParen, RightParan, LeftBrac, RightBrace, Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two char
    Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Less, LessEqual,

    // Literals
    Identifier, String, Number,
    
    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or, Print, Return, Super, This, True, Var, While,

    // End of file
    EOF,
}

// ** Keywords
lazy_static! {
    static ref KEYWORDS : HashMap<&'static str, TokenType> = HashMap::from ([
        ("and", TokenType::And),
        ("class", TokenType::Class),
        ("else", TokenType::Else),
        ("false", TokenType::False),
        ("for", TokenType::For),
        ("fun", TokenType::Fun),
        ("if", TokenType::If),
        ("nil", TokenType::Nil),
        ("or", TokenType::Or),
        ("print", TokenType::Print),
        ("return", TokenType::Return),
        ("super", TokenType::Super),
        ("this", TokenType::This),
        ("true", TokenType::True),
        ("var", TokenType::Var),
        ("while", TokenType::While),
    ]);
}

impl Copy for TokenType {}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: i32,
}

impl Display for Token {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{:?} {} {}]", self.token_type, self.lexeme, self.literal)
    }
}

#[derive(Debug)]
pub struct Scanner {
    source : String,
    tokens : Vec<Token>,

    start : i32,
    current : i32,
    line : i32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source,
            start: 0,
            current: 0,
            line: 1,
            tokens: vec![],
        }
    }
}

fn is_at_end (scanner : &mut Scanner) -> bool {
    return scanner.current >= scanner.source.len().try_into().unwrap();
}

fn scan_token (scanner : &mut Scanner) -> bool {
    let c = advance(scanner);
    let mut err_code = false;
    match c {
        // simple one char tokens
        '(' => _add_token(scanner, TokenType::LeftParen),
        ')' => _add_token(scanner, TokenType::RightParan),
        '{' => _add_token(scanner, TokenType::LeftBrac),
        '}' => _add_token(scanner, TokenType::RightBrace),
        ',' => _add_token(scanner, TokenType::Comma),
        '.' => _add_token(scanner, TokenType::Dot),
        '-' => _add_token(scanner, TokenType::Minus),
        '+' => _add_token(scanner, TokenType::Plus),
        ';' => _add_token(scanner, TokenType::Semicolon),
        '*' => _add_token(scanner, TokenType::Star),

        // two chat tokens
        '!' => if check_next(scanner, '=') { _add_token(scanner, TokenType::BangEqual) } else { _add_token(scanner, TokenType::Bang) },
        '=' => if check_next(scanner, '=') { _add_token(scanner, TokenType::EqualEqual) } else { _add_token(scanner, TokenType::Equal) },
        '<' => if check_next(scanner, '=') { _add_token(scanner, TokenType::LessEqual) } else { _add_token(scanner, TokenType::Less) },
        '>' => if check_next(scanner, '=') { _add_token(scanner, TokenType::GreaterEqual) } else { _add_token(scanner, TokenType::Greater) },
 
        // comments
        '/' => if check_next(scanner, '/') {
            while peek(scanner) != '\n' && !is_at_end(scanner) {
                advance(scanner);
            }
        } else {
            _add_token(scanner, TokenType::Slash);
        },

        '\n' => {scanner.line += 1;},
        
        // whitespace
        c if c.is_whitespace() => {}

        // Literals
        '"' => {string(scanner);}
        c if c.is_digit(10) => {number(scanner);}
        _ => {

            if c.is_ascii_alphabetic() {
                identifier(scanner);
            } else {
                err(scanner.line, "Unexpected character".to_string());
                err_code = true;
            }
        }
    }
    return err_code;
}

fn _add_token (scanner : &mut Scanner, token_type : TokenType)  {
    return add_token(scanner, token_type, "".to_string());
}

fn add_token (scanner : &mut Scanner, token_type : TokenType, literal : String) {
    let chrs = scanner.source.chars()
        .skip(scanner.start as usize)
        .take((scanner.current - scanner.start) as usize)
        .collect::<String>();

    scanner.tokens.push(Token{token_type, lexeme: chrs, literal, line: scanner.line});
}


fn advance (scanner : &mut Scanner) -> char {
    let c = scanner.source.chars().nth(scanner.current as usize).unwrap();
    scanner.current += 1;
    return c;
}

fn check_next (scanner : &mut Scanner, expected : char) -> bool {
    match scanner.source.chars().nth(scanner.current as usize) {
        Some(c) => {
            if c == expected {
                scanner.current += 1;
                return true;
            }
            return false;
        },
        None => return false,
    }
}

fn peek (scanner : &mut Scanner) -> char {
    match scanner.source.chars().nth(scanner.current as usize) {
        Some(c) => return c,
        None => return '\0',
    }
}

fn peek_next (scanner : &mut Scanner) -> char {
    if (scanner.current + 1) >= scanner.source.len().try_into().unwrap() {
        return '\0';
    }
    return scanner.source.chars().nth((scanner.current + 1) as usize).unwrap();
}

// helper functions to get parsing Literals
fn string (scanner : &mut Scanner) {
    while peek(scanner) != '"' && !is_at_end(scanner) {
        if (peek(scanner) == '\n') {
            scanner.line += 1;
        }
        advance(scanner);
    }
    if is_at_end(scanner) {
        err(scanner.line, "Unterminated string".to_string());
        return;
    }
    // close the string
    advance(scanner);

    let value = scanner.source.chars()
        .skip(scanner.start as usize + 1)
        .take((scanner.current - scanner.start - 2) as usize)
        .collect::<String>();
    add_token(scanner, TokenType::String, value);
}

// * Supports trailing dot
fn number (scanner : &mut Scanner) {
    let mut dot_offset = 0;
    while (peek(scanner).is_digit(10)) {
        advance(scanner);
    }
    let int_part = scanner.source.chars()
        .skip(scanner.start as usize)
        .take((scanner.current - scanner.start) as usize)
        .collect::<String>()
        .parse::<i32>().unwrap_or_default();
    if peek(scanner) == '.' {
        advance(scanner);
        dot_offset = scanner.current;
    } else {
        add_token(scanner, TokenType::Number, int_part.to_string());
        return;
    }

    while (peek(scanner).is_digit(10)) {
        advance(scanner);
    }

    let num_of_digits = scanner.current - dot_offset;

    let frac_part: i32 = scanner.source.chars()
        .skip(dot_offset as usize)
        .take((num_of_digits) as usize)
        .collect::<String>()
        .parse().unwrap_or_default();

    add_token(scanner, TokenType::Number, format!("{}.{}", int_part, frac_part));
}

fn identifier (scanner : &mut Scanner) {
    while (peek(scanner).is_alphanumeric()) {
        advance(scanner);
    }

    let text = scanner.source.chars()
        .skip(scanner.start as usize)
        .take((scanner.current - scanner.start) as usize)
        .collect::<String>();

    match KEYWORDS.get(&text.as_str()) {
        Some(token_type) => {
            _add_token(scanner, *token_type);
            return;
        },
        None => {
            add_token(scanner, TokenType::Identifier, text);
        }
    } 
}

pub trait ScanTokens {
    fn scan_tokens(&mut self) -> Vec<Token>;
}

impl ScanTokens for Scanner {
    fn scan_tokens(&mut self) -> Vec<Token> {
        while !is_at_end(self) {
            self.start = self.current;
            scan_token(self);
        }
        let eof = Token{token_type: TokenType::EOF, lexeme: "".to_string(), literal: "".to_string(), line: self.line};
        self.tokens.push(eof);
        return self.tokens.clone();
    }
}