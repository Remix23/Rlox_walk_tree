use crate::scanner::TokenType;
use crate::{expr::Visitor, scanner::LiteralType};
use crate::expr::{Binary, Conditional, Expr, Grouping, Literal, Unary};


pub struct Interpreter {}

impl Interpreter {
    pub fn evaluate (&mut self, expr : &Expr) -> LiteralType {
        expr.accept(self)
    }

    // Helpers:
    fn is_truthy (&mut self, literal : &LiteralType) -> bool {
        match literal {
            LiteralType::Nil => false,
            LiteralType::String(s) => !s.is_empty(),
            LiteralType::Number(n) => *n != 0.0,
            LiteralType::Bool(b) => *b,
        }
    }

    fn is_equal (&mut self, a : &LiteralType, b : &LiteralType) -> bool {
        match (a, b) {
            (LiteralType::Nil, LiteralType::Nil) => true,
            (LiteralType::String(s1), LiteralType::String(s2)) => s1 == s2,
            (LiteralType::Number(n1), LiteralType::Number(n2)) => n1 == n2,
            (LiteralType::Bool(b1), LiteralType::Bool(b2)) => b1 == b2,
            _ => false,
        }
    }

    fn report_run_time_error (&mut self) {
        todo!()
    }
}

impl Visitor<LiteralType> for Interpreter {

    fn visit_binary(&mut self, binary : &Binary) -> LiteralType {
        let left = self.evaluate(&binary.left);
        let right = self.evaluate(&binary.right);

        let operator = &binary.operator.token_type;

        match operator {
            TokenType::Minus => {
                match (left, right) {
                    (LiteralType::Number(l), LiteralType::Number(r)) => LiteralType::Number(l - r),
                    // TODO: Report error for not a number
                    _ => {todo!()}
                }
            }
            TokenType::Plus => {
                match (left, right) {
                    (LiteralType::Number(l), LiteralType::Number(r)) => LiteralType::Number(l + r),
                    (LiteralType::String(l), LiteralType::String(r)) => LiteralType::String(format!("{}{}", l, r)),
                    // TODO: Return runtime error for invalid types
                    _ => {todo!()}
                }
            }

            TokenType::Star => {
                match (left, right) {
                    (LiteralType::Number(l), LiteralType::Number(r)) => LiteralType::Number(l * r),
                    // TODO: Report error for not a number
                    _ => {todo!()}
                }
            }

            TokenType::Slash => {
                match (left, right) {
                    (LiteralType::Number(l), LiteralType::Number(r)) => LiteralType::Number(l / r),
                    // TODO: Report error for not a number
                    _ => {todo!()}
                }
            }

            // comparison operators
            TokenType::Greater => {
                match (left, right) {
                    (LiteralType::Number(l), LiteralType::Number(r)) => LiteralType::Bool(l > r),
                    // TODO: Report error for not a number
                    _ => {todo!()}
                }
            }
            TokenType::GreaterEqual => {
                match (left, right) {
                    (LiteralType::Number(l), LiteralType::Number(r)) => LiteralType::Bool(l >= r),
                    // TODO: Report error for not a number
                    _ => {todo!()}
                }
            }
            TokenType::Less => {
                match (left, right) {
                    (LiteralType::Number(l), LiteralType::Number(r)) => LiteralType::Bool(l < r),
                    // TODO: Report error for not a number
                    _ => {todo!()}
                }
            }
            TokenType::LessEqual => {
                match (left, right) {
                    (LiteralType::Number(l), LiteralType::Number(r)) => LiteralType::Bool(l <= r),
                    // TODO: Report error for not a number
                    _ => {todo!()}
                }
            }
            TokenType::EqualEqual => {
                LiteralType::Bool(self.is_equal(&left, &right))
            }
            TokenType::BangEqual => {
                LiteralType::Bool(!self.is_equal(&left, &right))
            }
            _ => {unreachable!()}
        }
    }

    fn visit_conditional(&mut self, conditional : &Conditional) -> LiteralType {
        let conditiona = self.evaluate(&conditional.condition);

        if self.is_truthy(&conditiona) {
            return self.evaluate(&conditional.then_branch);
        } else {
            return self.evaluate(&conditional.else_branch);
        }
    }

    fn visit_literal(&mut self, literal : &Literal) -> LiteralType {
        return literal.value.clone();
    }

    fn visit_grouping(&mut self, grouping : &Grouping) -> LiteralType {
        return self.evaluate(&grouping.expression);
    }

    fn visit_unary(&mut self, unary : &Unary) -> LiteralType {
        let right : LiteralType = self.evaluate(&unary.right);
        let operator = &unary.operator;

        match operator.token_type {
            TokenType::Minus => {
                match right {
                    LiteralType::Number(n) => LiteralType::Number(-n),
                    // TODO: Report error for not a number
                    _ => {todo!()}
                }
            },
            TokenType::Bang => {
                LiteralType::Bool(!self.is_truthy(&right))
            },
            _ => {unreachable!()}
        }
    }


}

