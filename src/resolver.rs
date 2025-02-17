// ? Static pass to resolve variable bindings
// ? Acording to the rule, that the from seeing the source code
// ? we know how resolve it

use std::collections::HashMap;
use std::env::var;
use std::thread::scope;

use crate::error_handler::err;
use crate::scanner::{Token, LiteralType};

use crate::stmt;
use crate::{
    expr::Expr,
    stmt::Stmt,
    expr::Visitor as ExprVisitor,
    stmt::Visitor as StmtVisitor,
    interpreter::Interpreter,
    error_handler::ResolverError,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FunctionType {
    Func,
    Method,
    None,
    INITIALIZER,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ClassType {
    Class,
    None,
    SUBCLASS,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoopType {
    Loop,
    None,
}

pub struct Resolver<'a> {
    pub scopes : Vec<HashMap<String, bool>>,     
    interpreter : &'a mut Interpreter,
    current_function : FunctionType,
    current_loop : LoopType,
    current_class : ClassType,

    had_error: bool,
}

impl<'a> Resolver<'a> {

    pub fn new (interpreter : &'a mut Interpreter) -> Resolver<'a> {
        Resolver {
            scopes : vec![],
            interpreter : interpreter,
            current_function : FunctionType::None,
            current_loop : LoopType::None,
            current_class : ClassType::None,
            had_error: false,
        }
    }

    pub fn resolve (&mut self, statements : &[Stmt]) {
        for stmt in statements {
            self.resolve_stmt(&stmt);
        }
    }

    pub fn had_error (&self) -> bool {
        self.had_error
    }

    fn resolve_stmt (&mut self, stmt : &Stmt) -> () {
        stmt.accept(self)
    }

    fn resolve_expr (&mut self, expr : &Expr) -> () {
        expr.accept(self)
    }

    fn begin_scope (&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope (&mut self) {
        self.scopes.pop();
    }

    fn declare (&mut self, name : &Token){
        if let Some (scope) = self.scopes.last_mut() {
            if scope.contains_key(&name.lexeme) {
                err(name.line, "Variable with this name already declared in this scope");
                self.had_error = true;
            }
            scope.insert(name.lexeme.clone(), false);
        }
    }

    fn define (&mut self, name : &Token) {
        if let Some (scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        }
    }

    fn resolve_local (&mut self, expr : &Expr, token : &Token) -> () {
        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(&token.lexeme) {
                self.interpreter.resolve(expr, self.scopes.len() - 1 - i);
            }
        }
        // dbg!(&self.scopes);
    }

    fn resolve_function (&mut self, function : &stmt::Function, typ : FunctionType) -> () {
        
        let enclosing_function = self.current_function;
        self.current_function = typ;
        
        self.begin_scope();
        for param in function.params.iter() {
            self.declare(param);
            self.define(param);
        }
        self.resolve(&function.body);
        self.end_scope(); 

        self.current_function = enclosing_function;
        
    }


}

impl<'a> ExprVisitor <()> for Resolver<'a> {

    fn visit_this(&mut self, this : &crate::expr::This) -> () {
        if self.current_class == ClassType::None {
            err(this.keyword.line, "Cannot use 'this' outside of a class");
            self.had_error = true;
            return;
        }
        self.resolve_local(&Expr::This(this.clone()), &this.keyword);
    }

    fn visit_superr(&mut self, superr : &crate::expr::Superr) -> () {
        
        if self.current_class == ClassType::None {
            err(superr.keyword.line, "Cannot use 'super' outside of a class");
            self.had_error = true;
        } else if self.current_class != ClassType::SUBCLASS {
            err(superr.keyword.line, "Cannot use 'super' in a class with no superclass");
            self.had_error = true;
        }

        self.resolve_local(&Expr::Superr(superr.clone()), &superr.keyword);
    }

    fn visit_variable(&mut self, variable : &crate::expr::Variable) -> () {
        
        if let Some (scope) = self.scopes.last() {
            if let Some (is_defined) = scope.get(&variable.name.lexeme) {
                if !is_defined {
                    err(variable.name.line, "Cannot read local variable in its own initializer");
                    self.had_error = true;
                }
            }
        }
        self.resolve_local(&Expr::Variable(variable.clone()), &variable.name);
    }

    fn visit_assigment(&mut self, assigment : &crate::expr::Assigment) -> () {
        self.resolve_expr(&assigment.value);
        self.resolve_local(&Expr::Assigment(assigment.clone()), &assigment.name);
        
    }

    fn visit_binary(&mut self, binary : &crate::expr::Binary) -> () {
        self.resolve_expr(&binary.left);
        self.resolve_expr(&binary.right);
        
    }

    fn visit_logical(&mut self, logical : &crate::expr::Logical) {
        self.resolve_expr(&logical.left);
        self.resolve_expr(&logical.right);
        
    }

    fn visit_conditional(&mut self, conditional : &crate::expr::Conditional){
        self.resolve_expr(&conditional.condition);
        self.resolve_expr(&conditional.then_branch);
        self.resolve_expr(&conditional.else_branch);
        
    }

    fn visit_call(&mut self, call : &crate::expr::Call) -> () {
        self.resolve_expr(&call.callee);
        for arg in call.arguments.iter() {
            self.resolve_expr(arg);
        }
        
    }

    fn visit_get(&mut self, get : &crate::expr::Get) -> () {
        self.resolve_expr(&get.object);
    }

    fn visit_set(&mut self, set : &crate::expr::Set) -> () {
        self.resolve_expr(&set.object);
        self.resolve_expr(&set.value);
    }

    fn visit_grouping(&mut self, grouping : &crate::expr::Grouping) -> () {
        self.resolve_expr(grouping.expression.as_ref());
        
    }

    fn visit_literal(&mut self, literal : &crate::expr::Literal) -> () {
        
    }

    fn visit_unary(&mut self, unary : &crate::expr::Unary) -> () {
        self.resolve_expr(&unary.right);
        
    }
}

impl<'a> StmtVisitor<()> for Resolver<'a> {
    fn visit_block(&mut self, block : &crate::stmt::Block) -> () {
        self.begin_scope();
        self.resolve (block.statements.as_slice());

        self.end_scope();
        
    }

    fn visit_var(&mut self, var : &stmt::Var) -> () {
        self.declare(&var.name);
        if let Some (init) = var.initializer.as_ref() {
            self.resolve_expr(init);
        }
        self.define(&var.name);
        
    }

    fn visit_function(&mut self, function : &stmt::Function) -> () {
        self.declare(&function.name);
        self.define(&function.name);

        self.resolve_function(function, FunctionType::Func);
    }

    fn visit_class(&mut self, class : &stmt::Class) -> () {

        let enclosing_class = self.current_class;
        self.current_class = ClassType::Class;

        self.declare(&class.name);
        self.define(&class.name);

        if let Some (Expr::Variable(sup)) = &class.SuperClass{

            self.current_class = ClassType::SUBCLASS;

            if sup.name.lexeme == class.name.lexeme {
                err(sup.name.line, "A class cannot inherit from itself");
                self.had_error = true;
            }

            self.resolve_expr(&class.SuperClass.as_ref().unwrap());

            self.begin_scope();

            self.scopes.last_mut().unwrap().insert("super".to_string(), true);
        }

        self.begin_scope();
        self.scopes.last_mut().unwrap().insert("this".to_string(), true);

        for method in class.methods.iter() {
            let decl = if method.name.lexeme == "init" {
                FunctionType::INITIALIZER
            } else {
                FunctionType::Method   
            };
            self.resolve_function(method, decl);
        }

        self.end_scope();

        if class.SuperClass.is_some() {
            self.end_scope();
        }

        self.current_class = enclosing_class;
    }

    fn visit_expression(&mut self, expression : &stmt::Expression) -> () {
        self.resolve_expr(&expression.expression);
        
    }

    fn visit_iff(&mut self, iff : &stmt::Iff) -> () {
        self.resolve_expr(&iff.condition);
        self.resolve_stmt(iff.then_branch.as_ref());
        if let Some (else_branch) = iff.else_branch.as_ref() {
            self.resolve_stmt(else_branch);
        }
        
    }

    fn visit_print(&mut self, print : &stmt::Print) -> () {

        self.resolve_expr(&print.expression);
        
    }

    fn visit_returnn(&mut self, returnn : &stmt::Returnn) -> () {
        if self.current_function == FunctionType::None {
            err(returnn.keyword.line, "Cannot use 'return' outside of a function");
            self.had_error = true;
        }
        
        if let Some (value) = &returnn.value {
        
            if self.current_class == ClassType::Class && self.current_function == FunctionType::INITIALIZER {
                err(returnn.keyword.line, "Cannot return a value from an initializer");
                self.had_error = true;

            }

            self.resolve_expr(value);
        }
    }

    fn visit_breakk(&mut self, breakk : &stmt::Breakk) -> () {
        if self.current_loop == LoopType::None {
            err(breakk.keyword.line, "Cannot use 'break' outside of a loop");
            self.had_error = true;
        }
        
    }

    fn visit_continuee(&mut self, continuee : &stmt::Continuee) -> () {
        if self.current_loop == LoopType::None {
            err(continuee.keyword.line, "Cannot use 'continue' outside of a loop");
            self.had_error = true;
        }
        
    }

    fn visit_whilee(&mut self, whilee : &stmt::Whilee) -> () {
        self.resolve_expr(whilee.condition.as_ref()); 

        self.current_loop = LoopType::Loop;
        self.resolve_stmt(whilee.body.as_ref());

        self.current_loop = LoopType::None;   
    }
}
