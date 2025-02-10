use crate::scanner::Token;

struct AST {}

enum Expr {
    Binary (Binary),
    Unary (Unary),

}

struct Unary {
    operator : Token,
    right : Box<Expr>,
}

struct Binary {
    left : Box<Expr>,
    operator : Token,
    right : Box<Expr>,
}

