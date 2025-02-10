use std::env;
use std::env::current_dir;
use std::fs;
use std::io::Write;
use std::path::{PathBuf};

use expr::Expr;
use scanner::{ScanTokens, TokenType};

// relative modules
use crate::scanner::{Scanner, Token};

pub mod scanner;
pub mod error_handler;
pub mod expr;
pub mod parser;
pub mod traits;

struct Lox {
    had_error: bool
}


fn run (s : String, rlox : &Lox) {
    let mut s = Scanner::new(s);
    let tokens = s.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }
}

fn run_file(file_name: PathBuf, rlox : &Lox) {
    let contents = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");
    run(contents, rlox);
}

fn run_prompt(rlox : &Lox) {
    println!("Running prompt");

    let exiting_code = ["exit", "quit", "q"];
    
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if exiting_code.contains(&input)  || input.len() == 0 {
                    println!("Exiting");
                    break;
                }
                run(input.to_string(), &rlox);
                if rlox.had_error {
                    break;
                }
            },
            Err(_) => {
                println!("Error reading input");
                break;
            }
        }
    }
}

fn main() {

    let args : Vec<String> = env::args().collect();

    let p = current_dir().unwrap();

    // create a new Lox instance

    let mut rlox = Lox {
        had_error: false
    };

    // test AST

    let expr = Expr::Binary(expr::Binary {
        left: Box::new(Expr::Unary(expr::Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: scanner::LiteralType::Nil,
                line: 1,
            },
            right: Box::new(Expr::Literal(expr::Literal {
                value: scanner::LiteralType::Number(123.0),
            })),
        })),
        operator: Token {
            token_type: TokenType::Star,
            lexeme: "*".to_string(),
            literal: scanner::LiteralType::Nil,
            line: 1,
        },
        right: Box::new(Expr::Grouping(expr::Grouping {
            expression: Box::new(Expr::Literal(expr::Literal {
                value: scanner::LiteralType::Nil,
            })),
        })),
    });

    let mut ast_printer = parser::AstPrinter {};
    ast_printer.print(&expr);
    
    let n_of_arguments = args.len();
    if n_of_arguments > 2 {
        println!("Usage: rlox <file_name>");
        return;
    } else if n_of_arguments == 2 {
        let file_path = p.join(&args[1]);
        run_file(file_path, &rlox);
    } else {
        run_prompt(&rlox);
    }
}
