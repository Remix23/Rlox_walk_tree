
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;

use crate::environemnt::Environemnt;
use crate::scanner::LiteralType;
use crate::interpreter::Interpreter;
use crate::stmt::{Function};
use crate::interpreter::Exit;

#[derive(Debug, Clone)]
pub enum Callable {
    LoxFunction (LoxFunction),
    NativeFunction (NativeFunction),
}

#[derive(Clone)]
pub struct LoxFunction {
    pub declaration : Box<Function>,
    pub closure : Rc<RefCell<Environemnt>>,
}
#[derive(Clone)]
pub struct NativeFunction {
    pub name : String,
    pub arity : i32,
    pub function : fn (&mut Interpreter, &Vec<LiteralType>) -> LiteralType,
}

pub trait LoxCallable {
    fn call (&self, interpreter : &mut Interpreter, arguments : &Vec<LiteralType>) -> Result<LiteralType, Exit>;
    fn arity (&self) -> i32;
}

impl LoxFunction {
    pub fn new (declaration : Function, closure : Rc<RefCell<Environemnt>>) -> LoxFunction {
        LoxFunction {
            declaration : Box::new(declaration),
            closure : closure,
        }
    }
}

impl LoxCallable for LoxFunction {
    fn call (&self, interpreter : &mut Interpreter, arguments : &Vec<LiteralType>) -> Result<LiteralType, Exit> {
        let mut env = Environemnt::new(Some(Rc::clone(&self.closure)));

        for (i, param) in self.declaration.params.iter().enumerate() {
            env.define(param.lexeme.clone(), arguments[i].clone());
        }
        let res = interpreter.execute_block(&self.declaration.body,  env, false);

        match res {
            Ok (_) => Ok(LiteralType::Nil),
            Err(e) => {
                match e {
                    Exit::Return(v) => {
                        return Ok(v.clone())
                    },
                    _ => {
                        dbg!(&e);
                        return Err(e)
                    }
                }
            }
        }
    }

    fn arity (&self) -> i32 {
        return self.declaration.params.len() as i32;
    }
}

impl Debug for LoxFunction {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<fn {}>", self.declaration.name.lexeme)
    }
}

impl Debug for NativeFunction {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<native fn {}>", self.name)
    }
}