
use crate::expr::{Expr, Visitor};

pub struct AstPrinter {}

impl Visitor<String> for AstPrinter {
    fn visit_binary(&mut self, binary : &crate::expr::Binary) -> String {
        return self.parenthesize(&binary.operator.lexeme, vec![&binary.left, &binary.right]);
    }
    fn visit_grouping(&mut self, grouping : &crate::expr::Grouping) -> String {
        return self.parenthesize(&"group".to_string(), vec![&grouping.expression]);
    }

    fn visit_literal(&mut self, literal : &crate::expr::Literal) -> String {
        return literal.value.to_string();
    }

    fn visit_unary(&mut self, unary : &crate::expr::Unary) -> String {
        return self.parenthesize(&unary.operator.lexeme, vec![&unary.right]);
    }
}

impl AstPrinter {
    pub fn print (&mut self, expr : &Expr) {
        println!("{}", expr.accept(self));
    }

    fn parenthesize (&mut self, name : &String, exprs : Vec<&Expr>) -> String {
        let mut s = String::new();
        s.push_str("(");
        s.push_str(name);
        for expr in exprs {
            s.push_str(format!(" {}", expr.accept(self)).as_str());
        }
        s.push_str(")");
        return s;
    }
}