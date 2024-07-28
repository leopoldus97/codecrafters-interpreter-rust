use std::collections::HashMap;

use crate::{
    ast::{
        expr::{
            self, assign::Assign, binary::Binary, call::Call, grouping::Grouping, literal::Literal,
            logical::Logical, unary::Unary, variable::Variable, Expr,
        },
        stmt::{
            self, block::Block, expression::Expression, function::Function, print::Print, r#if::If,
            r#return::Return, r#while::While, var::Var, Stmt,
        },
    },
    interpreter::Interpreter,
    scanner::token::{Object, Token},
    utils::error::Error,
};

#[derive(Clone, PartialEq)]
enum FunctionType {
    None,
    Function,
}

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        let scopes = vec![];
        Self {
            interpreter,
            scopes,
            current_function: FunctionType::None,
        }
    }

    pub fn resolve(&mut self, statements: &Vec<Box<dyn Stmt>>) -> Result<(), Error> {
        for stmt in statements {
            self.resolve_statement(stmt.as_ref())?;
        }
        Ok(())
    }

    fn resolve_statement(&mut self, stmt: &dyn Stmt) -> Result<(), Error> {
        stmt.accept(self)
    }

    fn resolve_expression(&mut self, expr: &dyn Expr<Object>) -> Result<(), Error> {
        expr.accept(self)
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        let scope = self.scopes.last_mut().unwrap();
        if scope.contains_key(name.lexeme()) {
            eprintln!("Variable with this name already declared in this scope.");
        }

        scope.insert(name.lexeme().to_owned(), false);
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        let scope = self.scopes.last_mut().unwrap();
        scope.insert(name.lexeme().to_owned(), true);
    }

    fn resolve_local(&mut self, expr: &dyn Expr<Object>, name: &Token) {
        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(name.lexeme()) {
                self.interpreter.resolve(expr, self.scopes.len() - 1 - i);
                return;
            }
        }
    }

    fn resolve_function(&mut self, function: &Function, function_type: FunctionType) {
        let enclosing_function = self.current_function.clone();
        self.current_function = function_type;

        self.begin_scope();
        for param in function.params() {
            self.declare(param);
            self.define(param);
        }
        self.resolve(&function.body()).unwrap();
        self.end_scope();

        self.current_function = enclosing_function;
    }
}

impl<'a> expr::Visitor<Object> for Resolver<'a> {
    fn visit_assign_expr(&mut self, expr: &Assign<Object>) -> Result<Object, Error> {
        self.resolve_expression(expr.value())?;
        self.resolve_local(expr, expr.name());
        Ok(Object::Nil)
    }

    fn visit_binary_expr(&mut self, expr: &Binary<Object>) -> Result<Object, Error> {
        self.resolve_expression(expr.left())?;
        self.resolve_expression(expr.right())?;
        Ok(Object::Nil)
    }

    fn visit_call_expr(&mut self, expr: &Call<Object>) -> Result<Object, Error> {
        self.resolve_expression(expr.callee())?;
        for argument in expr.arguments() {
            self.resolve_expression(argument.as_ref())?;
        }
        Ok(Object::Nil)
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping<Object>) -> Result<Object, Error> {
        self.resolve_expression(expr.expression())?;
        Ok(Object::Nil)
    }

    fn visit_literal_expr(&mut self, _: &Literal) -> Result<Object, Error> {
        Ok(Object::Nil)
    }

    fn visit_logical_expr(&mut self, expr: &Logical<Object>) -> Result<Object, Error> {
        self.resolve_expression(expr.left())?;
        self.resolve_expression(expr.right())?;
        Ok(Object::Nil)
    }

    fn visit_unary_expr(&mut self, expr: &Unary<Object>) -> Result<Object, Error> {
        self.resolve_expression(expr.right())?;
        Ok(Object::Nil)
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> Result<Object, Error> {
        if !self.scopes.is_empty()
            && self.scopes.last().is_some()
            && self.scopes.last().unwrap().get(expr.name().lexeme()) == false
        {
            eprintln!("Cannot read local variable in its own initializer.");
        }

        self.resolve_local(expr, expr.name());
        Ok(Object::Nil)
    }
}

impl<'a> stmt::Visitor for Resolver<'a> {
    fn visit_block_stmt(&mut self, stmt: &Block) -> Result<(), Error> {
        self.begin_scope();
        self.resolve(stmt.statements())?;
        self.end_scope();
        Ok(())
    }

    fn visit_expression_stmt(&mut self, stmt: &Expression) -> Result<(), Error> {
        self.resolve_expression(stmt.expression())?;
        Ok(())
    }

    fn visit_function_stmt(&mut self, stmt: &Function) -> Result<(), Error> {
        self.declare(stmt.name());
        self.define(stmt.name());

        self.resolve_function(stmt, FunctionType::Function);
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &If) -> Result<(), Error> {
        self.resolve_expression(stmt.condition())?;
        self.resolve_statement(stmt.then_branch())?;
        if let Some(ref else_branch) = stmt.else_branch() {
            self.resolve_statement(*else_branch)?;
        }
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &Print) -> Result<(), Error> {
        self.resolve_expression(stmt.expression())?;
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &Return) -> Result<(), Error> {
        if self.current_function == FunctionType::None {
            eprintln!(
                "{} Cannot return from top-level code.",
                stmt.keyword().lexeme()
            );
        }

        if let Some(ref value) = stmt.value() {
            self.resolve_expression(value.as_ref())?;
        }
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &Var) -> Result<(), Error> {
        self.declare(stmt.name());
        if let Some(ref initializer) = stmt.initializer() {
            self.resolve_expression(initializer.as_ref())?;
        }
        self.define(&stmt.name());
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &While) -> Result<(), Error> {
        self.resolve_expression(stmt.condition())?;
        self.resolve_statement(stmt.body())?;
        Ok(())
    }
}
