use crate::scanner::{Token, LiteralType};
#[derive(Debug, Clone)]
pub enum Expr {
    Binary (Binary),
    Logical (Logical),
    Call (Call),
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
}
#[derive(Debug, Clone)]
pub struct Logical {
    pub left : Box<Expr>,
    pub operator : Token,
    pub right : Box<Expr>,
}
#[derive(Debug, Clone)]
pub struct Call {
    pub callee : Box<Expr>,
    pub paren : Token,
    pub arguments : Vec<Expr>,
}
#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression : Box<Expr>,
}
#[derive(Debug, Clone)]
pub struct Literal {
    pub value : LiteralType,
}
#[derive(Debug, Clone)]
pub struct Unary {
    pub operator : Token,
    pub right : Box<Expr>,
}
#[derive(Debug, Clone)]
pub struct Conditional {
    pub condition : Box<Expr>,
    pub then_branch : Box<Expr>,
    pub else_branch : Box<Expr>,
}
#[derive(Debug, Clone)]
pub struct Variable {
    pub name : Token,
}
#[derive(Debug, Clone)]
pub struct Assigment {
    pub name : Token,
    pub value : Box<Expr>,
}
pub trait Visitor<T> {
    fn visit_binary(&mut self, binary : &Binary) -> T;
    fn visit_logical(&mut self, logical : &Logical) -> T;
    fn visit_call(&mut self, call : &Call) -> T;
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
            Expr::Grouping (grouping) => visitor.visit_grouping(grouping),
            Expr::Literal (literal) => visitor.visit_literal(literal),
            Expr::Unary (unary) => visitor.visit_unary(unary),
            Expr::Conditional (conditional) => visitor.visit_conditional(conditional),
            Expr::Variable (variable) => visitor.visit_variable(variable),
            Expr::Assigment (assigment) => visitor.visit_assigment(assigment),
          }
      }
}

