
use std::vec;

use crate::expr::{self, Assigment, Binary, Conditional, Expr, Grouping, Literal, Logical, Unary, Variable, Visitor};
use crate::scanner::{Token, TokenType, LiteralType};
use crate::{error_handler::*};
use crate::stmt::{Block, Expression, Iff, Print, Stmt, Var, Whilee, Breakk, Continuee};

pub struct Parser {
    tokens : Vec<Token>,
    current : usize,
    errors : Vec<ParseError>,
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

    fn visit_conditional(&mut self, conditional : &Conditional) -> String {
        return self.parenthesize(&"?:".to_string(), vec![&conditional.condition, &conditional.then_branch, &conditional.else_branch]);
    }
    fn visit_variable(&mut self, variable : &crate::expr::Variable) -> String {
        todo!()
    }

    fn visit_assigment(&mut self, assigment : &crate::expr::Assigment) -> String {
        todo!()
    }
    fn visit_logical(&mut self, logical : &Logical) -> String {
        todo!()
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
            errors: vec![],
        }
    }

    pub fn parse (&mut self) -> Result<Vec<Stmt>, ParseError>{
        let mut statements = vec![];

        while !self.is_at_end() {
            match self.declaration() {
                Some(stmt) => statements.push(stmt),
                None => {continue;}
            }
        }

        Ok(statements)
    }

    fn declaration (&mut self) -> Option<Stmt> {
        let stmt = if self.match_token(vec![TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };
        match stmt {
            Ok(stmt) => Some(stmt),
            Err(e) => {
                self.errors.push(e);
                self.synchronise();
                None
            }
        }
    }

    fn var_declaration (&mut self) -> Result<Stmt, ParseError> {
        let token = self.consume(TokenType::Identifier, "Expect a variable name")?;

        let mut initializer= None;
        if self.match_token(vec![TokenType::Equal]) {
            initializer = Some(self.expression()?);
        } 
        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration")?;
        Ok (
            Stmt::Var(Var {
                name : token,
                initializer :  initializer
            })
        )
    }

    fn statement (&mut self) -> Result<Stmt, ParseError> {
        match self.peek().token_type {
            TokenType::Print => {
                self.advance(); 
                return self.print_statement()
            }
            TokenType::LeftBrac => {
                self.advance();
                let block = self.block()?;
                return Ok(Stmt::Block(Block {
                    statements : block
                }));
            }
            TokenType::If => {
                self.advance();
                self.if_statement()
            }
            TokenType::While => {
                self.advance();
                self.while_statement()
            }
            TokenType::For => {
                self.advance();
                self.for_statement()
            }
            TokenType::Break => {
                self.advance();
                self.break_statement()
            }
            TokenType::Continue => {
                self.advance();
                self.continue_statement()
            }
            _ => {self.expression_statement()}
        }
    }

    fn print_statement (&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value")?;
        Ok(Stmt::Print(Print {
            expression : Box::new(value)
        }))
    }

    fn block (&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = vec![];

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            match self.declaration() {
                Some(stmt) => statements.push(stmt),
                None => {continue;}
            }
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block")?;
        return Ok(statements);
    }

    fn if_statement (&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'")?;
        let condition = self.expression()?;

        self.consume(TokenType::RightParan, "Expect ')' after condition")?;

        let then_branch = self.statement()?;
        if self.match_token(vec![TokenType::Else]) {
            let else_branch = self.statement()?;
            return Ok(Stmt::Iff(Iff {
                condition : Box::new(condition),
                then_branch : Box::new(then_branch),
                else_branch : Some(Box::new(else_branch))
            }))
        }
        Ok(Stmt::Iff(Iff {
            condition : Box::new(condition),
            then_branch : Box::new(then_branch),
            else_branch : None
        }))
    }

    fn while_statement (&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'")?;

        let condition = self.expression()?;
        self.consume(TokenType::RightParan, "Expect ')' after condition")?;

        let body = self.statement()?;

        Ok (Stmt::Whilee(Whilee {
            condition : Box::new(condition),
            body : Box::new(body),
            is_for : false
        }))
    }

    fn for_statement (&mut self) -> Result<Stmt, ParseError> {
        
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'")?;

        let initializer = if self.match_token(vec![TokenType::Semicolon]) {
            None
        } else if self.match_token(vec![TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };
        
        let condition = if !self.check(TokenType::Semicolon) {
            self.expression()?
        } else {
            expr::Expr::Literal(Literal {
                value : LiteralType::Bool(true)
            })
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition")?;

        let increment = if !self.check(TokenType::RightParan) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RightParan, "Expect ')' after for clauses")?;

        let mut body = self.statement()?;
        
        // * Desugaring for loop
        if let Some(increment) = increment {
            body = Stmt::Block(Block {
                statements : vec![body, Stmt::Expression(Expression {
                    expression : Box::new(increment)
                })]
            })
        }
        // * constructing the while loop

        body = Stmt::Whilee(Whilee {
            condition : Box::new(condition),
            body : Box::new(body),
            is_for : true
        });

        if let Some(initializer) = initializer {
            body = Stmt::Block(Block {
                statements : vec![initializer, body]
            })
        }

        return Ok(body);
    }

    fn expression_statement (&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value")?;
        Ok(Stmt::Expression(Expression {
            expression : Box::new(value)
        }))
    }

    fn break_statement (&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::Semicolon, "Expect ';' after break")?;
        Ok(Stmt::Breakk(Breakk {}))
    }

    fn continue_statement (&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::Semicolon, "Expect ';' after continue")?;
        Ok(Stmt::Continuee(Continuee {}))
    }

    fn expression (&mut self) -> Result<Expr, ParseError> {
        self.comma()
    }

    // TODO: add support for coma oprator 
    // * Done
    fn comma (&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.second_level()?;

        while self.match_token(vec![TokenType::Comma]) {
            let operator = self.previous();
            let right = self.second_level()?;
            expr = Expr::Binary(Binary {
                left : Box::new(expr),
                operator : operator,
                right : Box::new(right)
            });
        }
        Ok(expr)
    }

    // TODO: add support for ternary operator;
    fn second_level (&mut self) -> Result<Expr, ParseError> {
        let condition = self.logical_or()?;
        match self.peek().token_type {
            TokenType::QuestionMark => {
                self.advance();
                let then_branch = self.second_level()?;
                self.consume(TokenType::Colon, "Expect ':' after then branch")?;
                let else_branch = self.second_level()?;
                return Ok(Expr::Conditional(Conditional {
                    condition : Box::new(condition),
                    then_branch : Box::new(then_branch),
                    else_branch : Box::new(else_branch)
                }));
            }
            TokenType::Equal => {
                // * Assigment
                self.advance();
                let eq = self.previous();
                let value = self.second_level()?;
                match condition {
                    Expr::Variable(v) => {
                        let name = v.name;
                        return Ok(Expr :: Assigment(Assigment {
                            name : name,
                            value : Box::new(value)
                        }))
                    }
                    _ => {
                        return Err(parse_error(&eq, "Invalid assigment target"));
                    }
                }
            }
            _ => {}
        }
        
        Ok(condition)
    }

    
    fn logical_or (&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.logical_and ()?;

        while self.match_token(vec![TokenType::Or]) {
            let operator = self.previous();
            let right = self.logical_and()?;
            expr = Expr::Logical(Logical {
                left : Box::new(expr),
                operator : operator,
                right : Box::new(right)
            })
        }
        return Ok(expr);
    }
    fn logical_and (&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        while self.match_token(vec![TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical(Logical {
                left : Box::new(expr),
                operator : operator,
                right : Box::new(right)
            })
        }

        Ok (expr)
    }

    fn equality (&mut self) -> Result<Expr, ParseError> {
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

    fn comparison (&mut self) -> Result<Expr, ParseError> {
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

    fn term (&mut self) -> Result<Expr, ParseError> {
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

    fn factor (&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_token(vec![TokenType::Slash, TokenType::Star, TokenType::Percentage]) {
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

    fn unary (&mut self) -> Result<Expr, ParseError> {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous ();
            let right = self.unary()?;
            return Ok(Expr::Unary(Unary {
                operator : operator,
                right : Box::new(right)
            }))
        }

        self.primary()
    }

    fn primary (&mut self) -> Result<Expr, ParseError> {
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

                self.consume(TokenType::RightParan, "Expect ')' after expression")?;

                Ok(Expr::Grouping(Grouping {
                    expression : Box::new(expr)
                }))
            }
            TokenType::Identifier => {
                self.advance();
                Ok (Expr::Variable(Variable {
                    name : self.previous()
                }))
            }
            _ => {
                Err(parse_error(&self.peek(), "Expect expression"))
            }
        }
    }

    fn consume (&mut self, token : TokenType, msg : &str ) -> Result<Token, ParseError> {
        if self.check(token) { return Ok(self.advance());}

        let curr = self.peek();

        // parsing error
        Err(parse_error(&curr, msg))
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