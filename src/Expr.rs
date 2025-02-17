use crate::scanner::{Token, LiteralType};
use std::hash::Hash;
#[derive(Debug, Clone)]
pub enum Expr {
    Binary (Binary),
    Logical (Logical),
    Call (Call),
    Get (Get),
    Set (Set),
    Superr (Superr),
    This (This),
    Grouping (Grouping),
    Literal (Literal),
    Unary (Unary),
    Conditional (Conditional),
    Variable (Variable),
    Assigment (Assigment),
}
#[derive(Debug, Clone)]
pub struct Binary {
    pub left : Box<Expr>,
    pub operator : Token,
    pub right : Box<Expr>,
    pub uuid : usize
}
#[derive(Debug, Clone)]
pub struct Logical {
    pub left : Box<Expr>,
    pub operator : Token,
    pub right : Box<Expr>,
    pub uuid : usize
}
#[derive(Debug, Clone)]
pub struct Call {
    pub callee : Box<Expr>,
    pub paren : Token,
    pub arguments : Vec<Expr>,
    pub uuid : usize
}
#[derive(Debug, Clone)]
pub struct Get {
    pub object : Box<Expr>,
    pub name : Token,
    pub uuid : usize
}
#[derive(Debug, Clone)]
pub struct Set {
    pub object : Box<Expr>,
    pub name : Token,
    pub value : Box<Expr>,
    pub uuid : usize
}
#[derive(Debug, Clone)]
pub struct Superr {
    pub keyword : Token,
    pub method : Token,
    pub uuid : usize
}
#[derive(Debug, Clone)]
pub struct This {
    pub keyword : Token,
    pub uuid : usize
}
#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression : Box<Expr>,
    pub uuid : usize
}
#[derive(Debug, Clone)]
pub struct Literal {
    pub value : LiteralType,
    pub uuid : usize
}
#[derive(Debug, Clone)]
pub struct Unary {
    pub operator : Token,
    pub right : Box<Expr>,
    pub uuid : usize
}
#[derive(Debug, Clone)]
pub struct Conditional {
    pub condition : Box<Expr>,
    pub then_branch : Box<Expr>,
    pub else_branch : Box<Expr>,
    pub uuid : usize
}
#[derive(Debug, Clone)]
pub struct Variable {
    pub name : Token,
    pub uuid : usize
}
#[derive(Debug, Clone)]
pub struct Assigment {
    pub name : Token,
    pub value : Box<Expr>,
    pub uuid : usize
}
pub trait Visitor<T> {
    fn visit_binary(&mut self, binary : &Binary) -> T;
    fn visit_logical(&mut self, logical : &Logical) -> T;
    fn visit_call(&mut self, call : &Call) -> T;
    fn visit_get(&mut self, get : &Get) -> T;
    fn visit_set(&mut self, set : &Set) -> T;
    fn visit_superr(&mut self, superr : &Superr) -> T;
    fn visit_this(&mut self, this : &This) -> T;
    fn visit_grouping(&mut self, grouping : &Grouping) -> T;
    fn visit_literal(&mut self, literal : &Literal) -> T;
    fn visit_unary(&mut self, unary : &Unary) -> T;
    fn visit_conditional(&mut self, conditional : &Conditional) -> T;
    fn visit_variable(&mut self, variable : &Variable) -> T;
    fn visit_assigment(&mut self, assigment : &Assigment) -> T;
}
impl Expr {
    pub fn accept<T>(&self, visitor : &mut dyn Visitor<T>) -> T {
        match self {
            Expr::Binary (binary) => visitor.visit_binary(binary),
            Expr::Logical (logical) => visitor.visit_logical(logical),
            Expr::Call (call) => visitor.visit_call(call),
            Expr::Get (get) => visitor.visit_get(get),
            Expr::Set (set) => visitor.visit_set(set),
            Expr::Superr (superr) => visitor.visit_superr(superr),
            Expr::This (this) => visitor.visit_this(this),
            Expr::Grouping (grouping) => visitor.visit_grouping(grouping),
            Expr::Literal (literal) => visitor.visit_literal(literal),
            Expr::Unary (unary) => visitor.visit_unary(unary),
            Expr::Conditional (conditional) => visitor.visit_conditional(conditional),
            Expr::Variable (variable) => visitor.visit_variable(variable),
            Expr::Assigment (assigment) => visitor.visit_assigment(assigment),
          }
      }
    pub fn get_uuid(&self) -> usize {
        match self {
            Expr::Binary (e) => e.uuid,
            Expr::Logical (e) => e.uuid,
            Expr::Call (e) => e.uuid,
            Expr::Get (e) => e.uuid,
            Expr::Set (e) => e.uuid,
            Expr::Superr (e) => e.uuid,
            Expr::This (e) => e.uuid,
            Expr::Grouping (e) => e.uuid,
            Expr::Literal (e) => e.uuid,
            Expr::Unary (e) => e.uuid,
            Expr::Conditional (e) => e.uuid,
            Expr::Variable (e) => e.uuid,
            Expr::Assigment (e) => e.uuid,
          }
      }
}
impl Hash for Expr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_uuid().hash(state);
    }
}
impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.get_uuid() == other.get_uuid()
    }
}
impl Eq for Expr {
}

