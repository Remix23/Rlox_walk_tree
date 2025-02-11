
use core::panic;
use std::vec;

use crate::expr::{Binary, Expr, Grouping, Literal, Unary, Visitor};
use crate::scanner::{Token, TokenType, LiteralType};

pub struct Parser {
    tokens : Vec<Token>,
    current : usize,
}

pub struct AstPrinter {}

impl Visitor<String> for AstPrinter {
    fn visit_binary(&mut self, binary : &crate::expr::Binary) -> String {
        return self.parenthesize(&binary.operator.lexeme, vec![&binary.left, &binary.right]);
    }
    fn visit_grouping(&mut self, grouping : &crate::expr::Grouping) -> String {
        return self.parenthesize(&"group".to_string(), vec![&grouping.expression]);
    }

    fn visit_literal(&mut self, literal : &crate::expr::Literal) -> String {
        return literal.value.to_string();
    }

    fn visit_unary(&mut self, unary : &crate::expr::Unary) -> String {
        return self.parenthesize(&unary.operator.lexeme, vec![&unary.right]);
    }
}

impl AstPrinter {
    pub fn print (&mut self, expr : &Expr) {
        println!("{}", expr.accept(self));
    }

    fn parenthesize (&mut self, name : &String, exprs : Vec<&Expr>) -> String {
        let mut s = String::new();
        s.push_str("(");
        s.push_str(name);
        for expr in exprs {
            s.push_str(format!(" {}", expr.accept(self)).as_str());
        }
        s.push_str(")");
        return s;
    }
}

impl Parser {
    pub fn new (tokens : Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
        }
    }

    fn expression (&mut self) -> Expr {
        return self.equality();
    }

    fn equality (&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_token(vec![TokenType::EqualEqual, TokenType::BangEqual]) {
            let operator = self.previous ();
            let right = self.comparison();
            expr = Expr::Binary(Binary {
                left : Box::new(expr),
                operator : operator, 
                right : Box::new(right)
            });
        }
        return expr;
    }

    fn comparison (&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_token(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.term();

            expr = Expr::Binary(Binary {
                left : Box::new(expr), 
                operator : operator,
                right : Box::new(right)
            })
        }

        return expr;
    }

    fn term (&mut self) -> Expr {
        let mut expr = self.factor ();

        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor ();
            expr = Expr::Binary (Binary {
                left : Box::new(expr),
                operator : operator,
                right : Box::new(right)
            })
        }
        return expr;
    }

    fn factor (&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous ();
            let right = self.unary ();
            expr = Expr::Binary (Binary {
                left : Box::new(expr),
                operator : operator,
                right : Box::new(right)
            })
        }

        return expr;
    }

    fn unary (&mut self) -> Expr {
        let expr = self.primary ();

        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous ();
            let right = self.unary ();
            return Expr::Unary(Unary {
                operator : operator,
                right : Box::new(right)
            })
        }

        return expr;
    }

    fn primary (&mut self) -> Expr {
        match self.peek().token_type  {
            TokenType::False => {
                self.advance();
                return Expr::Literal(Literal {
                    value : LiteralType::Bool(false)
                })
            }
            TokenType::True => {
                self.advance();
                return Expr::Literal(Literal {
                    value : LiteralType::Bool(true)
                })
            }
            TokenType::Nil => {
                self.advance();
                return Expr::Literal(Literal {
                    value : LiteralType::Nil
                })
            }
            TokenType::Number => {
                self.advance();
                return Expr::Literal(Literal {
                    value : self.previous().literal.clone()
                })
            }
            TokenType::String => {
                self.advance();
                return Expr::Literal(Literal {
                    value : LiteralType::String(self.previous().literal.to_string())
                })
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression();
                return Expr::Grouping(Grouping {
                    expression : Box::new(expr)
                }
                )
            }
            _ => {return Expr::Literal(Literal {value : LiteralType::Nil})}
        }
    }

    fn consume (&mut self, token : TokenType, msg : &str) -> Token {
        if self.check(token) { return self.advance();}

        panic!("{msg}");
    }

    fn match_token (&mut self, tokens : Vec<TokenType>) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check (&mut self, token : TokenType) -> bool{
        if self.is_at_end() {return false;}

        return self.peek().token_type == token;
    }

    fn advance (&mut self) -> Token {
        if !self.is_at_end() {self.current += 1;}

        self.previous()
    }

    fn previous (&mut self) -> Token {
        self.tokens[self.current - 1].clone() 
    }

    fn peek (&mut self) -> Token {
        return self.tokens[self.current].clone();
    }
    fn is_at_end (&mut self) -> bool {
        return self.peek().token_type == TokenType::EOF;
    }
    
}