use crate::scanner::{Token, LiteralType};
pub enum Expr {
    Binary (Binary),
    Grouping (Grouping),
    Literal (Literal),
    Unary (Unary),
}
pub struct Binary {
    pub left : Box<Expr>,
    pub operator : Token,
    pub right : Box<Expr>,
}
pub struct Grouping {
    pub expression : Box<Expr>,
}
pub struct Literal {
    pub value : LiteralType,
}
pub struct Unary {
    pub operator : Token,
    pub right : Box<Expr>,
}
pub trait Visitor<T> {
    fn visit_binary(&mut self, binary : &Binary) -> T;
    fn visit_grouping(&mut self, grouping : &Grouping) -> T;
    fn visit_literal(&mut self, literal : &Literal) -> T;
    fn visit_unary(&mut self, unary : &Unary) -> T;
}
impl Expr {
    pub fn accept<T>(&self, visitor : &mut dyn Visitor<T>) -> T {
        match self {
            Expr::Binary (binary) => visitor.visit_binary(binary),
            Expr::Grouping (grouping) => visitor.visit_grouping(grouping),
            Expr::Literal (literal) => visitor.visit_literal(literal),
            Expr::Unary (unary) => visitor.visit_unary(unary),
          }
      }
}

