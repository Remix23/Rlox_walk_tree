

#[cfg(test)]
mod test {
    use crate::parser::Parser;
    use crate::scanner::{Scanner, Token};
    use crate::interpreter::Interpreter;

    #[test]
    fn simple_scan () {
        let mut s = Scanner::new("1 + 2".to_string());
        let tokens = s.scan_tokens();
        if let Ok(tokens) = tokens {
            assert_eq!(tokens.len(), 4);
        }
            
    }
    #[test]
    fn addition () {
        let mut s = Scanner::new("print 1 + 2;".to_string());
        let tokens = s.scan_tokens();
        if let Ok(tokens) = tokens {
            let mut parser = Parser::new(tokens);
            match parser.parse() {
                Ok(stmts) => {
                    let mut interpreter = Interpreter::new();
                    let _ = interpreter.interpret(stmts, false);
                }
                Err(_) => {
                    println!("Error parsing expression");
                }
            }
        }    
    }
}