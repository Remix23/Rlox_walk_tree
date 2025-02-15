use crate::expr::Expr;
use crate::scanner::{Token, LiteralType};
pub enum Stmt {
    Expression (Expression),
    Print (Print),
    Var (Var),
    Block (Block),
    Iff (Iff),
    Whilee (Whilee),
    Breakk (Breakk),
    Continuee (Continuee),
}
pub struct Expression {
    pub expression : Box<Expr>,
}
pub struct Print {
    pub expression : Box<Expr>,
}
pub struct Var {
    pub name : Token,
    pub initializer : Option<Expr>,
}
pub struct Block {
    pub statements : Vec<Stmt>,
}
pub struct Iff {
    pub condition : Box<Expr>,
    pub then_branch : Box<Stmt>,
    pub else_branch : Option<Box<Stmt>>,
}
pub struct Whilee {
    pub condition : Box<Expr>,
    pub body : Box<Stmt>,
    pub is_for : bool,
}
pub struct Breakk {
}
pub struct Continuee {
}
pub trait Visitor<T> {
    fn visit_expression(&mut self, expression : &Expression) -> T;
    fn visit_print(&mut self, print : &Print) -> T;
    fn visit_var(&mut self, var : &Var) -> T;
    fn visit_block(&mut self, block : &Block) -> T;
    fn visit_iff(&mut self, iff : &Iff) -> T;
    fn visit_whilee(&mut self, whilee : &Whilee) -> T;
    fn visit_breakk(&mut self, breakk : &Breakk) -> T;
    fn visit_continuee(&mut self, continuee : &Continuee) -> T;
}
impl Stmt {
    pub fn accept<T>(&self, visitor : &mut dyn Visitor<T>) -> T {
        match self {
            Stmt::Expression (expression) => visitor.visit_expression(expression),
            Stmt::Print (print) => visitor.visit_print(print),
            Stmt::Var (var) => visitor.visit_var(var),
            Stmt::Block (block) => visitor.visit_block(block),
            Stmt::Iff (iff) => visitor.visit_iff(iff),
            Stmt::Whilee (whilee) => visitor.visit_whilee(whilee),
            Stmt::Breakk (breakk) => visitor.visit_breakk(breakk),
            Stmt::Continuee (continuee) => visitor.visit_continuee(continuee),
          }
      }
}

