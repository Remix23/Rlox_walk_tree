
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::scanner::{Token, TokenType};
use crate::{expr, scanner::LiteralType, stmt};
use crate::expr::{Binary, Conditional, Expr, Grouping, Literal, Unary, Variable};
use crate::stmt::{Expression, Print, Stmt};
use crate::error_handler::{err, RuntimeError};
use crate::environemnt::Environemnt;
use crate::loxcallable::{LoxCallable, Callable, LoxFunction, NativeFunction};
// TODO: Add runtime error handling

// TODO: Implement the following:
// * Anonymus functions

pub struct Interpreter {
    pub environment : Rc<RefCell<Environemnt>>,
    pub globals : Rc<RefCell<Environemnt>>,
    pub locals : HashMap<Expr, usize>,
    loop_break : bool,
    loop_continue : bool,
    in_loop : bool,
}
#[derive(Debug)]
pub enum Exit {
    Return (LiteralType),
    RuntimeError (RuntimeError)
}

impl Interpreter {

    pub fn new () -> Interpreter {
        let global = Rc::new(RefCell::new(Environemnt::new(None)));

        let mut i = Interpreter {
            environment : Rc::clone(&global),
            globals : Rc::clone(&global),
            loop_break : false,
            loop_continue : false,
            in_loop : false,
            locals : HashMap::new(),
        };
        i.define_global_funcs();
        i
    }

    pub fn resolve (&mut self, expr : &Expr, depth : usize){
        self.locals.insert(expr.clone(), depth);
    }

    fn define_global_funcs (&mut self) {
        // clock
        let clock_func = Callable::NativeFunction(NativeFunction {
            name : "clock".to_string(),
            arity : 0,
            function : |_interpreter, _args| {
                let time = std::time::SystemTime::now();
                let since_the_epoch = time.duration_since(std::time::UNIX_EPOCH).unwrap();
                LiteralType::Number(since_the_epoch.as_secs_f64())
            }
        });
        self.globals.borrow_mut().define("clock".to_string(), LiteralType::Callable(clock_func));

        // TODO: add file handling | buffer handling
    }

    pub fn evaluate (&mut self, expr : &Expr) -> Result<LiteralType, Exit> {
        expr.accept( self)
    }

    fn execute (&mut self, stmt : &Stmt) -> Result<(), Exit> {
        stmt.accept(self)
    }

    pub fn interpret (&mut self, stmts : Vec<Stmt>, repl : bool) -> Result<(), Exit> {
        for stmt in stmts {
        
            if repl {
                match stmt {
                    Stmt::Expression(e) => {
                        let val = self.evaluate(&e.expression)?;
                        self.print_val(&val);
                        continue;
                    },
                    _ => {}
                }
            }
            self.execute(&stmt)?;
        }
        Ok(())
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
            _ => {
                // TODO: print for lox callables
                println!("{:?}", value);
            }
        }
    }

    pub fn execute_block (&mut self, statements : &Vec<Stmt>, environment : Environemnt, is_loop : bool) -> Result<(), Exit> {
        let previous = Rc::clone(&self.environment);

        self.environment = Rc::new(RefCell::new(environment));

        // println!("Entering");
        // dbg!(&self.environment);

        for stmt in statements {
            if is_loop && (self.loop_break || self.loop_continue) {
                break;
            }
            let res = self.execute(stmt);
            match &res {
                Ok (_) => {},
                Err(e) => {
                    self.environment = previous;
                    match e {
                        Exit::Return(_) => {
                            return res;
                        },
                        _ => {
                            dbg!(e);
                            return res;
                        }
                    }
                }
            }

        }
        self.environment = previous;

        // println!("Exiting");
        // dbg!(&self.environment);
        Ok(())
    }

    fn look_up_variable (&mut self, name : Token, expr : &Expr) -> Result<LiteralType, Exit> {
        let distance = self.locals.get(expr);

        match distance {
            Some (d) => {
                Ok(self.environment.borrow_mut().get_at(*d as i32, name.lexeme).unwrap())
            },
            None => {
                //get at global scope
                match self.globals.borrow_mut().get_at(0, name.lexeme.clone()) {
                    Some (val) => {
                        return Ok(val.clone());
                    },
                    None => {
                        return Err(Exit::RuntimeError(RuntimeError {
                            token : name.clone(),
                            message : format!("Undefined variable '{}'", name.lexeme.clone())
                        }));
                    }
                }
            }
        }

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
        _ => {
            false
        }
    }
}

impl expr::Visitor<Result<LiteralType, Exit>> for Interpreter {

    fn visit_binary(&mut self, binary : &Binary) -> Result<LiteralType, Exit> {
        let left = self.evaluate(&binary.left)?;
        let right = self.evaluate(&binary.right)?;

        let operator = &binary.operator.token_type;

        match operator {
            TokenType::Minus => {
                match (left, right) {
                    (LiteralType::Number(l), LiteralType::Number(r)) => Ok(LiteralType::Number(l - r)),
                    // TODO: Report error for not a number
                    _ => {
                        Err(Exit::RuntimeError(RuntimeError {
                            token : binary.operator.clone(),
                            message : "Operands must be numbers".to_string()
                        }))
                    }
                }
            }
            TokenType::Plus => {
                match (left, right) {
                    (LiteralType::Number(l), LiteralType::Number(r)) => Ok(LiteralType::Number(l + r)),
                    (LiteralType::String(l), LiteralType::String(r)) => Ok(LiteralType::String(format!("{}{}", l, r))),
                    
                    // TODO: Casting to string
                    // * DONE
                    (LiteralType::Number(n), LiteralType::String(s)) => {
                        Ok(LiteralType::String(format!("{}{}", n, s)))
                    }
                    (LiteralType::String(s), LiteralType::Number(n)) => {
                        Ok(LiteralType::String(format!("{}{}", s, n)))
                    }
                    // TODO: Return runtime error for invalid types
                    _ => {
                        Err(Exit::RuntimeError(RuntimeError {
                            token : binary.operator.clone(),
                            message : "Operands must be two numbers or two strings".to_string()
                        }))
                    }
                }
            }

            TokenType::Star => {
                match (left, right) {
                    (LiteralType::Number(l), LiteralType::Number(r)) => Ok(LiteralType::Number(l * r)),
                    // TODO: Report error for not a number
                    _ => {
                        Err(Exit::RuntimeError(RuntimeError {
                            token : binary.operator.clone(),
                            message : "Operands must be numbers".to_string()
                        }))
                    }
                }
            } 

            TokenType::Slash => {
                match (left, right) {
                    (LiteralType::Number(l), LiteralType::Number(r)) => {
                        // TODO: Return runtime error for division by zero

                        if r == 0.0 {
                            return Err(Exit::RuntimeError(RuntimeError {
                                token : binary.operator.clone(),
                                message : "Division by zero".to_string()
                            }));
                        }

                        Ok(LiteralType::Number(l / r))
                    },
                    // TODO: Report error for not a number
                    _ => {
                        Err(Exit::RuntimeError(RuntimeError {
                            token : binary.operator.clone(),
                            message : "Operands must be numbers".to_string()
                        }))
                    }
                }
            }

            // modulo operator
            TokenType::Percentage => {
                match (left, right) {
                    (LiteralType::Number(l), LiteralType::Number(r)) => {
                        // try casting into integer
                        Ok(LiteralType::Number(l % r))
                    },
                    _ => {
                        // TODO: Report error for not a number
                        Err(Exit::RuntimeError(RuntimeError {
                            token : binary.operator.clone(),
                            message : "Operands must be numbers".to_string()
                        }))
                    }
                }
            }
            // comparison operators
            TokenType::Greater => {
                match (left, right) {
                    (LiteralType::Number(l), LiteralType::Number(r)) => Ok(LiteralType::Bool(l > r)),
                    // TODO: Report error for not a number
                    _ => {
                        Err(Exit::RuntimeError(RuntimeError {
                            token : binary.operator.clone(),
                            message : "Operands must be numbers".to_string()
                        }))
                    }
                }
            }
            TokenType::GreaterEqual => {
                match (left, right) {
                    (LiteralType::Number(l), LiteralType::Number(r)) => Ok(LiteralType::Bool(l >= r)),
                    // TODO: Report error for not a number
                    _ => {
                        Err(Exit::RuntimeError(RuntimeError {
                            token : binary.operator.clone(),
                            message : "Operands must be numbers".to_string()
                        }))
                    }
                }
            }
            TokenType::Less => {
                match (left, right) {
                    (LiteralType::Number(l), LiteralType::Number(r)) => Ok(LiteralType::Bool(l < r)),
                    // TODO: Report error for not a number
                    _ => {
                        Err(Exit::RuntimeError(RuntimeError {
                            token : binary.operator.clone(),
                            message : "Operands must be numbers".to_string()
                        }))
                    }
                }
            }
            TokenType::LessEqual => {
                match (left, right) {
                    (LiteralType::Number(l), LiteralType::Number(r)) => Ok(LiteralType::Bool(l <= r)),
                    // TODO: Report error for not a number
                    _ => {
                        Err(Exit::RuntimeError(RuntimeError {
                            token : binary.operator.clone(),
                            message : "Operands must be numbers".to_string()
                        }))
                    }
                }
            }
            TokenType::EqualEqual => {
                Ok(LiteralType::Bool(self.is_equal(&left, &right)))
            }
            TokenType::BangEqual => {
                Ok(LiteralType::Bool(!self.is_equal(&left, &right)))
            }
            TokenType::Comma => {
                Ok(right)
            }
            _ => {panic!("{:?}", binary)}
        }
    }

    fn visit_conditional(&mut self, conditional : &Conditional) -> Result<LiteralType, Exit> {
        let conditiona = self.evaluate(&conditional.condition)?;

        if is_truthy(&conditiona) {
            return self.evaluate(&conditional.then_branch);
        } else {
            return self.evaluate(&conditional.else_branch);
        }
    }

    fn visit_literal(&mut self, literal : &Literal) -> Result<LiteralType, Exit> {
        Ok(literal.value.clone())
    }

    fn visit_grouping(&mut self, grouping : &Grouping) -> Result<LiteralType, Exit> {
        self.evaluate(&grouping.expression)
    }

    fn visit_unary(&mut self, unary : &Unary) -> Result<LiteralType, Exit> {
        let right : LiteralType = self.evaluate(&unary.right)?;
        let operator = &unary.operator;

        match operator.token_type {
            TokenType::Minus => {
                match right {
                    LiteralType::Number(n) => Ok(LiteralType::Number(-n)),
                    // TODO: Report error for not a number
                    _ => {
                        Err(Exit::RuntimeError(RuntimeError {
                            token : operator.clone(),
                            message : "Operand must be a number".to_string()
                        }))
                    }
                }
            },
            TokenType::Bang => {
                Ok(LiteralType::Bool(!is_truthy(&right)))
            },
            _ => {unreachable!()}
        }
    }

    fn visit_variable(&mut self, variable : &expr::Variable) -> Result<LiteralType, Exit> {
        return self.look_up_variable(variable.name.clone(), &Expr::Variable(variable.clone()))
    }
    fn visit_assigment(&mut self, assigment : &expr::Assigment) -> Result<LiteralType, Exit> {
        let value = self.evaluate(&assigment.value)?;
        let name = &assigment.name.lexeme;

        let distance = self.locals.get(&Expr::Assigment(assigment.clone()));
        match distance {
            Some (d) => {
                self.environment.borrow_mut().assign_at(*d as i32, name.clone(), value.clone())
            },
            None => {
                self.globals.borrow_mut().assign_at(0, name.clone(), value.clone())
            }
        };

        Ok(value)
    }

    fn visit_logical(&mut self, logical : &expr::Logical) -> Result<LiteralType, Exit> {
        let left = self.evaluate(&logical.left)?;

        match logical.operator.token_type {
            TokenType::Or => {
                if is_truthy(&left) {
                    return Ok(left);
                }
            },
            TokenType::And => {
                if !is_truthy(&left) {
                    return Ok(left);
                }
            },
            _ => {unreachable!()}
        }
        return self.evaluate(&logical.right);
    }

    fn visit_call(&mut self, call : &expr::Call) -> Result<LiteralType, Exit> {
        let callee = self.evaluate(&call.callee)?;
        let mut args = vec![];
        for arg in &call.arguments {
            args.push(self.evaluate(arg)?);
        }

        if let LiteralType::Callable(Callable::LoxFunction(function)) = callee {
            if args.len() as i32 != function.arity() {
                // TODO: Report error for invalid number of arguments
                println!("Expected {} arguments but got {}", function.arity(), args.len());
                return Err(Exit::RuntimeError(RuntimeError {
                    token : call.paren.clone(),
                    message : format!("Expected {} arguments but got {}", function.arity(), args.len())
                }));
            }   
            function.call(self, &args)
        } else {
            Err(Exit::RuntimeError(
                RuntimeError {
                    token : call.paren.clone(),
                    message : "Can only call functions and classes".to_string()
                }
            ))
        }
    }
}

impl stmt::Visitor<Result<(), Exit>> for Interpreter {
    fn visit_expression(&mut self, expression : &Expression) -> Result<(), Exit> {
        self.evaluate(&expression.expression)?;
        Ok(())
    }

    fn visit_print(&mut self, print : &Print) -> Result<(), Exit> {
        let value: LiteralType = self.evaluate(&print.expression)?;
        self.print_val(&value);
        Ok(())
    }

    fn visit_var(&mut self, var : &stmt::Var) -> Result<(), Exit> {
        let value = match &var.initializer {
            Some (expr) => self.evaluate(expr)?,
            None => LiteralType::Nil,
        };

        self.environment.borrow_mut().define(var.name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_block(&mut self, block : &stmt::Block) -> Result<(), Exit> {
        let stmts = &block.statements;
        let environment = Environemnt::new(Some(self.environment.clone()));
        self.execute_block(stmts, environment, self.in_loop)?;
        Ok(())
    }

    fn visit_iff(&mut self, iff : &stmt::Iff) -> Result<(), Exit> {
        let condition = self.evaluate(&iff.condition)?;
        if is_truthy(&condition) {
            self.execute(&iff.then_branch)?;
        } else {
            if let Some (else_branch) = &iff.else_branch {
                self.execute(&else_branch)?;
            }     
        }
        Ok(())
    }
    fn visit_whilee(&mut self, whilee : &stmt::Whilee) -> Result<(), Exit> {

        let condition = &whilee.condition;

        while is_truthy(& self.evaluate(condition)?) {

            self.in_loop = true;

            self.execute(&whilee.body)?;

            if self.loop_break {
                self.loop_break = false;
                break;
            }
        }

        self.in_loop = false;
        Ok(())
    }

    fn visit_breakk(&mut self, _break : &stmt::Breakk) -> Result<(), Exit> {
        self.loop_break = true;
        Ok(())
    }

    fn visit_continuee(&mut self, _continue : &stmt::Continuee) -> Result<(), Exit> {
        self.loop_continue = true;
        Ok(())
    }

    fn visit_function(&mut self, function : &stmt::Function) -> Result<(), Exit> {
        let f = Callable::LoxFunction(LoxFunction::new(function.clone(), Rc::clone(&self.environment)));
        self.environment.borrow_mut().define(function.name.lexeme.clone(), LiteralType::Callable(f));
        Ok(())
    }
    fn visit_returnn(&mut self, returnn : &stmt::Returnn) -> Result<(), Exit> {
        let value = match &returnn.value {
            Some (expr) => self.evaluate(expr)?,
            None => LiteralType::Nil,
        };
        Err(Exit::Return(value))
    }
}