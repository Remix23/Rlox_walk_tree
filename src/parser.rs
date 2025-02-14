
use core::panic;
use std::vec;

use crate::expr::{Binary, Expr, Grouping, Literal, Unary, Visitor};
use crate::scanner::{Token, TokenType, LiteralType};
use crate::error_handler::*;

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

    pub fn parse (&mut self) -> Result<Expr, LoxError>{
        self.expression()
    }
    fn expression (&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn equality (&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.match_token(vec![TokenType::EqualEqual, TokenType::BangEqual]) {
            let operator = self.previous ();
            let right = self.comparison()?;
            expr = Expr::Binary(Binary {
                left : Box::new(expr),
                operator : operator, 
                right : Box::new(right)
            });
        }
        return Ok(expr);
    }

    fn comparison (&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while self.match_token(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.term()?;

            expr = Expr::Binary(Binary {
                left : Box::new(expr), 
                operator : operator,
                right : Box::new(right)
            })
        }

        return Ok(expr);
    }

    fn term (&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary (Binary {
                left : Box::new(expr),
                operator : operator,
                right : Box::new(right)
            })
        }
        return Ok(expr);
    }

    fn factor (&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous ();
            let right = self.unary ()?;
            expr = Expr::Binary (Binary {
                left : Box::new(expr),
                operator : operator,
                right : Box::new(right)
            })
        }

        return Ok(expr);
    }

    fn unary (&mut self) -> Result<Expr, LoxError> {
        let expr = self.primary()?;

        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous ();
            let right = self.unary()?;
            return Ok(Expr::Unary(Unary {
                operator : operator,
                right : Box::new(right)
            }))
        }

        Ok(expr)
    }

    fn primary (&mut self) -> Result<Expr, LoxError> {
        match self.peek().token_type  {
            TokenType::False => {
                self.advance();
                Ok(Expr::Literal(Literal {
                    value : LiteralType::Bool(false)
                }))
            }
            TokenType::True => {
                self.advance();
                Ok(Expr::Literal(Literal {
                    value : LiteralType::Bool(true)
                }))
            }
            TokenType::Nil => {
                self.advance();
                Ok(Expr::Literal(Literal {
                    value : LiteralType::Nil
                }))
            }
            TokenType::Number => {
                self.advance();
                Ok(Expr::Literal(Literal {
                    value : self.previous().literal.clone()
                }))
            }
            TokenType::String => {
                self.advance();
                Ok(Expr::Literal(Literal {
                    value : LiteralType::String(self.previous().literal.to_string())
                }))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;

                Ok(Expr::Grouping(Grouping {
                    expression : Box::new(expr)
                }))
            }
            _ => {Ok(Expr::Literal(Literal {value : LiteralType::Nil}))}
        }
    }

    fn consume (&mut self, token : TokenType, msg :String ) -> Result<Token, LoxError> {
        if self.check(token) { return Ok(self.advance());}

        let curr = self.peek();

        // parsing error
        Err(parse_error(&curr, msg.as_str()))
    }

    fn synchronise (&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {return; }

            match self.peek().token_type {
                // get all statement tokens
                TokenType::Class | TokenType::Fun | TokenType::Var | TokenType::For | TokenType::If | TokenType::While | TokenType::Print | TokenType::Return => return,
                _ => {}
            }
            self.advance();
        }
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