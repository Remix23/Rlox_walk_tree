

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::environemnt;
    use crate::parser::Parser;
    use crate::scanner::{Scanner, ScanTokens};
    use crate::interpreter::Interpreter;

    #[test]
    fn simple_scan () {
        let mut s = Scanner::new("1 + 2".to_string());
        let tokens = s.scan_tokens();
        assert_eq!(tokens.len(), 4);
    }
    #[test]
    fn addition () {
        let mut s = Scanner::new("print 1 + 2;".to_string());
        let tokens = s.scan_tokens();
        let mut parser = Parser::new(tokens);
        let global = environemnt::Environemnt::new(None);
        match parser.parse() {
            Ok(stmts) => {
                let mut interpreter = Interpreter::new(Rc::new(RefCell::new(global)));
                interpreter.interpret(stmts, false);
            }
            Err(e) => {
                println!("Error parsing expression");
            }
        }
    }
}