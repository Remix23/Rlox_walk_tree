use std::rc::Rc;
use std::cell::{Ref, RefCell};

use crate::scanner::TokenType;
use crate::{expr, scanner::LiteralType, stmt};
use crate::expr::{Binary, Conditional, Expr, Grouping, Literal, Unary};
use crate::stmt::{Expression, Print, Stmt, Visitor};
use crate::environemnt::Environemnt;

// TODO: Add runtime error handling
pub struct Interpreter {
    environment : Rc<RefCell<Environemnt>>,
}

impl Interpreter {

    pub fn new (global :  Rc<RefCell<Environemnt>>) -> Interpreter {
        Interpreter {
            environment : global,
        }
    }

    pub fn evaluate (&mut self, expr : &Expr) -> LiteralType {
        expr.accept( self)
    }

    fn execute (&mut self, stmt : &Stmt) {
        stmt.accept(self);
    }

    pub fn interpret (&mut self, stmts : Vec<Stmt>, repl : bool) {
        for stmt in stmts {
        
            if repl {
                match stmt {
                    Stmt::Expression(e) => {
                        let val = self.evaluate(&e.expression);
                        self.print_val(&val);
                        continue;
                    },
                    _ => {}
                }
            }
            self.execute(&stmt);
        }
    }

    // Helpers:


    fn is_equal (&mut self, a : &LiteralType, b : &LiteralType) -> bool {
        match (a, b) {
            (LiteralType::Nil, LiteralType::Nil) => true,
            (LiteralType::String(s1), LiteralType::String(s2)) => s1 == s2,
            (LiteralType::Number(n1), LiteralType::Number(n2)) => n1 == n2,
            (LiteralType::Bool(b1), LiteralType::Bool(b2)) => b1 == b2,
            _ => false,
        }
    }

    fn print_val (&self, value : &LiteralType) {
        match value {
            LiteralType::String(s) => println!("{}", s),
            LiteralType::Number(n) => println!("{}", n),
            LiteralType::Bool(b) => println!("{}", b),
            LiteralType::Nil => println!("nil"),
        }
    }

    fn execute_block (&mut self, statements : &Vec<Stmt>, environment : Environemnt) {
        let previous = Rc::clone(&self.environment);

        self.environment = Rc::new(RefCell::new(environment));
        for stmt in statements {
            self.execute(stmt);
        }
        self.environment = previous;
    }

    fn report_run_time_error (&mut self) {
        todo!()
    }
}

fn is_truthy (literal : &LiteralType) -> bool {
    match literal {
        LiteralType::Nil => false,
        LiteralType::String(s) => !s.is_empty(),
        LiteralType::Number(n) => *n != 0.0,
        LiteralType::Bool(b) => *b,
    }
}

impl expr::Visitor<LiteralType> for Interpreter {

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
                    
                    // TODO: Casting to string
                    // * DONE
                    (LiteralType::Number(n), LiteralType::String(s)) => {
                        LiteralType::String(format!("{}{}", n, s))
                    }
                    (LiteralType::String(s), LiteralType::Number(n)) => {
                        LiteralType::String(format!("{}{}", s, n))
                    }
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
                    (LiteralType::Number(l), LiteralType::Number(r)) => {
                        // TODO: Return runtime error for division by zero

                        LiteralType::Number(l / r)
                    },
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

        if is_truthy(&conditiona) {
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
                LiteralType::Bool(!is_truthy(&right))
            },
            _ => {unreachable!()}
        }
    }

    fn visit_variable(&mut self, variable : &expr::Variable) -> LiteralType {
        let name = &variable.name.lexeme;
        match self.environment.borrow_mut().get(name.clone()) {
            Some(value) => value.clone(),
            None => {
                // TODO: Report error for undefined variable
                println!("Undefined variable '{}'", name);
                return LiteralType::Nil;
            }
        }
    }
    fn visit_assigment(&mut self, assigment : &expr::Assigment) -> LiteralType {
        let value = self.evaluate(&assigment.value);
        let name = &assigment.name.lexeme;
        self.environment.borrow_mut().assign(name.clone(), value.clone());
        return value;
    }

    fn visit_logical(&mut self, logical : &expr::Logical) -> LiteralType {
        let left = self.evaluate(&logical.left);

        match logical.operator.token_type {
            TokenType::Or => {
                if is_truthy(&left) {
                    return left;
                }
            },
            TokenType::And => {
                if !is_truthy(&left) {
                    return left;
                }
            },
            _ => {unreachable!()}
        }
        return self.evaluate(&logical.right);
    }
}

impl stmt::Visitor<()> for Interpreter {
    fn visit_expression(&mut self, expression : &Expression) {
        self.evaluate(&expression.expression);
    }

    fn visit_print(&mut self, print : &Print) {
        let value: LiteralType = self.evaluate(&print.expression);
        self.print_val(&value);
    }

    fn visit_var(&mut self, var : &stmt::Var) {
        let value = match &var.initializer {
            Some (expr) => self.evaluate(expr),
            None => LiteralType::Nil,
        };

        self.environment.borrow_mut().define(var.name.lexeme.clone(), value);
    }
    fn visit_block(&mut self, block : &stmt::Block) -> () {
        let stmts = &block.statements;
        let environment = Environemnt::new(Some(self.environment.clone()));

        self.execute_block(stmts, environment);
    }

    fn visit_iff(&mut self, iff : &stmt::Iff) -> () {
        let condition = self.evaluate(&iff.condition);
        if is_truthy(&condition) {
            self.execute(&iff.then_branch);
        } else {
            if let Some (else_branch) = &iff.else_branch {
                self.execute(&else_branch);
            }     
        }
    }
    fn visit_whilee(&mut self, whilee : &stmt::Whilee) -> () {

        let condition = &whilee.condition;

        while is_truthy(& self.evaluate(condition)) {
            self.execute(&whilee.body);
        }
    }
}