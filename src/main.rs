use std::cell::{RefCell};
use std::env;
use std::env::current_dir;
use std::fs;
use std::io::Write;
use std::path::{PathBuf};
use std::rc::Rc;

use scanner::{ScanTokens};

// relative modules
use crate::scanner::{Scanner};

pub mod scanner;
pub mod error_handler;
pub mod expr;
pub mod parser;
pub mod traits;
pub mod interpreter;
pub mod stmt;
pub mod environemnt;

pub mod tests;

struct Lox {
    had_error: bool,
    repl : bool,
    interpreter : interpreter::Interpreter,
}

impl Lox {

    fn run (&mut self, s : String) {

        // let mut printer = parser::AstPrinter {};
        let mut s = Scanner::new(s);
        let tokens = s.scan_tokens();

        let mut parser = parser::Parser::new(tokens);
        match parser.parse() {
            Ok(stmts) => {

                self.interpreter.interpret(stmts, self.repl);
            },
            Err(e) => {
                println!("Error parsing expression");
            }
        }
    }
    
    fn run_file(&mut self, file_name: PathBuf) {
        self.repl = false;
        let contents = fs::read_to_string(file_name)
            .expect("Something went wrong reading the file");
        self.run(contents);
    }

    fn run_prompt(&mut self) {
        self.repl = true;
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
                    self.run(input.to_string());
                    if self.had_error {
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

}
fn main() {

    let args : Vec<String> = env::args().collect();

    let p = current_dir().unwrap();

    // create a new Lox instance

    let global_env = environemnt::Environemnt::new(None);

    let mut rlox = Lox {
        had_error: false,
        repl: false,
        interpreter: interpreter::Interpreter::new(Rc::new(RefCell::new(global_env))),
    };
    
    let n_of_arguments = args.len();
    if n_of_arguments > 2 {
        println!("Usage: rlox <file_name>");
        return;
    } else if n_of_arguments == 2 {
        let file_path = p.join(&args[1]);
        rlox.run_file(file_path);
    } else {
        rlox.run_prompt();
    }
}
