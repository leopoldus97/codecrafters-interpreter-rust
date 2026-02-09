use std::{collections::HashMap, rc::Rc};

use crate::{
    ast::{
        expr::{
            self, assign::Assign, binary::Binary, call::Call, get::Get, grouping::Grouping,
            literal::Literal, logical::Logical, set::Set, super_keyword::Super, this::This,
            unary::Unary, variable::Variable, Expr,
        },
        stmt::{
            self, block::Block, class::Class, expression::Expression, function::Function,
            print::Print, r#if::If, r#return::Return, r#while::While, var::Var, Stmt,
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
    Initializer,
    Method,
}

#[derive(Clone, PartialEq)]
enum ClassType {
    None,
    Class,
    Subclass,
}

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
    current_class: ClassType,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        let scopes = vec![];
        Self {
            interpreter,
            scopes,
            current_function: FunctionType::None,
            current_class: ClassType::None,
        }
    }

    pub fn resolve(&mut self, statements: &Vec<Rc<dyn Stmt>>) -> Result<Object, Box<Error>> {
        for stmt in statements {
            self.resolve_statement(stmt.as_ref())?;
        }
        Ok(Object::Nil)
    }

    fn resolve_statement(&mut self, stmt: &dyn Stmt) -> Result<Object, Box<Error>> {
        stmt.accept(self)
    }

    fn resolve_expression(&mut self, expr: &dyn Expr) -> Result<Object, Box<Error>> {
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

    fn resolve_local(&mut self, expr: Rc<dyn Expr>, name: &Token) {
        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(name.lexeme()) {
                let _ = self
                    .interpreter
                    .resolve(expr, (self.scopes.len() - 1 - i) as u8);
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
        self.resolve(function.body()).unwrap();
        self.end_scope();

        self.current_function = enclosing_function;
    }
}

impl<'a> expr::Visitor for Resolver<'a> {
    fn visit_assign_expr(&mut self, expr: &Assign) -> Result<Object, Box<Error>> {
        self.resolve_expression(expr.value())?;
        self.resolve_local(Rc::new(expr.clone()), expr.name());
        Ok(Object::Nil)
    }

    fn visit_binary_expr(&mut self, expr: &Binary) -> Result<Object, Box<Error>> {
        self.resolve_expression(expr.left())?;
        self.resolve_expression(expr.right())?;
        Ok(Object::Nil)
    }

    fn visit_call_expr(&mut self, expr: &Call) -> Result<Object, Box<Error>> {
        self.resolve_expression(expr.callee())?;
        for argument in expr.arguments() {
            self.resolve_expression(argument.as_ref())?;
        }
        Ok(Object::Nil)
    }

    fn visit_get_expr(&mut self, expr: &Get) -> Result<Object, Box<Error>> {
        self.resolve_expression(expr.object().as_ref())?;
        Ok(Object::Nil)
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<Object, Box<Error>> {
        self.resolve_expression(expr.expression())?;
        Ok(Object::Nil)
    }

    fn visit_literal_expr(&mut self, _: &Literal) -> Result<Object, Box<Error>> {
        Ok(Object::Nil)
    }

    fn visit_logical_expr(&mut self, expr: &Logical) -> Result<Object, Box<Error>> {
        self.resolve_expression(expr.left())?;
        self.resolve_expression(expr.right())?;
        Ok(Object::Nil)
    }

    fn visit_set_expr(&mut self, expr: &Set) -> Result<Object, Box<Error>> {
        self.resolve_expression(expr.value().as_ref())?;
        self.resolve_expression(expr.object().as_ref())?;
        Ok(Object::Nil)
    }

    fn visit_super_expr(&mut self, expr: &Super) -> Result<Object, Box<Error>> {
        if self.current_class == ClassType::None {
            eprintln!("Can't use 'super' outside of a class.");
        } else if self.current_class != ClassType::Subclass {
            eprintln!("Can't use 'super' in a class with no superclass.");
        }

        self.resolve_local(Rc::new(expr.clone()), expr.keyword());
        Ok(Object::Nil)
    }

    fn visit_this_expr(&mut self, expr: &This) -> Result<Object, Box<Error>> {
        if self.current_class == ClassType::None {
            eprintln!("Cannot use 'this' outside of a class.");
        }

        self.resolve_local(Rc::new(expr.clone()), expr.keyword());
        Ok(Object::Nil)
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> Result<Object, Box<Error>> {
        self.resolve_expression(expr.right())?;
        Ok(Object::Nil)
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> Result<Object, Box<Error>> {
        if !self.scopes.is_empty()
            && self.scopes.last().is_some()
            && self
                .scopes
                .last()
                .unwrap()
                .get(expr.name().lexeme())
                .is_some_and(|b| b == &false)
        {
            eprintln!("Cannot read local variable in its own initializer.");
        }

        self.resolve_local(Rc::new(expr.clone()), expr.name());
        Ok(Object::Nil)
    }
}

impl<'a> stmt::Visitor for Resolver<'a> {
    fn visit_block_stmt(&mut self, stmt: &Block) -> Result<Object, Box<Error>> {
        self.begin_scope();
        self.resolve(stmt.statements())?;
        self.end_scope();
        Ok(Object::Nil)
    }

    fn visit_class_stmt(&mut self, stmt: &Class) -> Result<Object, Box<Error>> {
        let enclosing_class = self.current_class.clone();
        self.current_class = ClassType::Class;

        self.declare(stmt.name());
        self.define(stmt.name());

        if let Some(superclass) = stmt.superclass() {
            if stmt.name().lexeme() == superclass.name().lexeme() {
                eprintln!("{} A class cannot inherit from itself.", superclass.name());
            }
            self.current_class = ClassType::Subclass;
            self.resolve_expression(superclass)?;
        }

        if stmt.superclass().is_some() {
            self.begin_scope();
            self.scopes
                .last_mut()
                .unwrap()
                .insert(String::from("super"), true);
        }

        self.begin_scope();
        self.scopes
            .last_mut()
            .unwrap()
            .insert(String::from("this"), true);

        for method in stmt.methods() {
            let mut declaration = FunctionType::Method;
            if method.name().lexeme() == "init" {
                declaration = FunctionType::Initializer;
            }

            self.resolve_function(method, declaration)
        }

        self.end_scope();

        if stmt.superclass().is_some() {
            self.end_scope();
        }

        self.current_class = enclosing_class;
        Ok(Object::Nil)
    }

    fn visit_expression_stmt(&mut self, stmt: &Expression) -> Result<Object, Box<Error>> {
        self.resolve_expression(stmt.expression())?;
        Ok(Object::Nil)
    }

    fn visit_function_stmt(&mut self, stmt: &Function) -> Result<Object, Box<Error>> {
        self.declare(stmt.name());
        self.define(stmt.name());

        self.resolve_function(stmt, FunctionType::Function);
        Ok(Object::Nil)
    }

    fn visit_if_stmt(&mut self, stmt: &If) -> Result<Object, Box<Error>> {
        self.resolve_expression(stmt.condition())?;
        self.resolve_statement(stmt.then_branch())?;
        if let Some(else_branch) = stmt.else_branch() {
            self.resolve_statement(else_branch)?;
        }
        Ok(Object::Nil)
    }

    fn visit_print_stmt(&mut self, stmt: &Print) -> Result<Object, Box<Error>> {
        self.resolve_expression(stmt.expression())?;
        Ok(Object::Nil)
    }

    fn visit_return_stmt(&mut self, stmt: &Return) -> Result<Object, Box<Error>> {
        if self.current_function == FunctionType::None {
            eprintln!(
                "{} Cannot return from top-level code.",
                stmt.keyword().lexeme()
            );
        }

        if let Some(ref value) = stmt.value() {
            if self.current_function == FunctionType::Initializer {
                eprintln!(
                    "{} Cannot return a value from an initializer.",
                    stmt.keyword()
                );
            }

            self.resolve_expression(value.as_ref())?;
        }
        Ok(Object::Nil)
    }

    fn visit_var_stmt(&mut self, stmt: &Var) -> Result<Object, Box<Error>> {
        self.declare(stmt.name());
        if let Some(ref initializer) = stmt.initializer() {
            self.resolve_expression(initializer.as_ref())?;
        }
        self.define(stmt.name());
        Ok(Object::Nil)
    }

    fn visit_while_stmt(&mut self, stmt: &While) -> Result<Object, Box<Error>> {
        self.resolve_expression(stmt.condition())?;
        self.resolve_statement(stmt.body())?;
        Ok(Object::Nil)
    }
}
