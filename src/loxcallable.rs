
use std::collections::HashMap;
use std::fmt::{Display, Debug};
use std::rc::Rc;
use std::cell::{Ref, RefCell};

use crate::environemnt::Environemnt;
use crate::error_handler::RuntimeError;
use crate::scanner::{LiteralType, Token};
use crate::interpreter::Interpreter;
use crate::stmt::{Function};
use crate::interpreter::Exit;
use crate::Lox;

#[derive(Debug, Clone)]
pub enum Callable {
    LoxFunction (LoxFunction),
    NativeFunction (NativeFunction),
    LoxCLass (LoxCLass),
    LoxInstance (Rc<RefCell<LoxInstance>>),
}

#[derive(Clone, Debug)]
pub struct LoxFunction {
    pub declaration : Box<Function>,
    pub closure : Rc<RefCell<Environemnt>>,
}
#[derive(Debug, Clone)]
pub struct LoxCLass {
    pub name : String,
    pub methods : HashMap<String, LoxFunction>,

}

#[derive(Debug, Clone)]
pub struct LoxInstance {
    class : Rc<LoxCLass>,
    fields : HashMap<String, LiteralType>,
}
#[derive(Clone, Debug)]
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

    pub fn bind (&self, instance : Rc<RefCell<LoxInstance>>) -> LoxFunction {
        let env = Rc::new(RefCell::new(Environemnt::new (
            Some (Rc::clone(&self.closure))
        )));
        env.borrow_mut().define(
            "this".to_string(), 
            LiteralType::Callable(Callable::LoxInstance(instance))
        );

        LoxFunction {
            declaration : self.declaration.clone(),
            closure : env
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

impl LoxCLass {
    pub fn new (name : String, methods : HashMap<String, LoxFunction>) -> LoxCLass {
        LoxCLass {
            name : name,
            methods : methods,
        }
    }

    pub fn find_method (&self, name : String) -> Option<&LoxFunction> {
        if self.methods.contains_key(&name) {
            return Some(self.methods.get(&name).unwrap());
        }
        None
    }
}

// constructor for classes
impl LoxCallable for LoxCLass {
    fn call (&self, interpreter : &mut Interpreter, arguments : &Vec<LiteralType>) -> Result<LiteralType, Exit> {
        let inst = Rc::new(RefCell::new(
            LoxInstance {
                class : Rc::new(self.clone()),
                fields : HashMap::new(),
            }
        ));

        if let Some (init) = self.find_method("init".to_string()) {
            let outer = init.bind(Rc::clone(&inst));
            outer.call(interpreter, arguments)?;
        }
        Ok (LiteralType::Callable(Callable::LoxInstance(
            inst
        )))
    }

    fn arity (&self) -> i32 {
        if let Some (init) = self.find_method("init".to_string()) {
            return init.arity();
        }
        return 0;
    }
}

impl LoxInstance {
    pub fn get (&self, name : &Token) -> Result<LiteralType, Exit> {
        if self.fields.contains_key(&name.lexeme) {
            return Ok(self.fields.get(&name.lexeme).unwrap().clone());
        }

        if let Some(method) = self.class.find_method(name.lexeme.clone()) {
            let func = method.bind(Rc::new(RefCell::new(self.to_owned())));
            return Ok(LiteralType::Callable(Callable::LoxFunction(func)));
        }

        Err (Exit::RuntimeError(RuntimeError {
            token : name.clone(),
            message : format!("Undefined property '{}'", name.lexeme)
        }))
    }
    pub fn set (&mut self, name : &Token, value : LiteralType) {
        self.fields.insert(name.lexeme.clone(), value);
    }
}

impl Display for LoxFunction {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<fn: {}>", self.declaration.name.lexeme)
    }
}

impl Display for NativeFunction {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<native fn: {}>", self.name)
    }
}

impl Display for LoxCLass {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<class: {}>", self.name)
    }
}

impl Display for Callable {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Callable::LoxCLass(c) => c.to_string(),
            Callable::LoxFunction(f) => f.to_string(),
            Callable::NativeFunction(f) => f.to_string(),
            Callable::LoxInstance(i) => i.borrow().to_string(),
        };
        write!(f, "{}", s)
    }
}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<class: {} instance>", self.class.name, )
    }
}