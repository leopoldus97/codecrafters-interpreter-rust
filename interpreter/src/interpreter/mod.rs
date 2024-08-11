pub mod callable;
pub mod environment;
mod error;

use std::{
    cell::RefCell,
    collections::HashMap,
    hash::{Hash, Hasher},
    ops::Neg,
    rc::Rc,
};

use callable::{clock::ClockFn, Callable, Fun};
use environment::Environment;
use error::runtime_error;

use crate::{
    ast::{
        expr::{
            self, assign::Assign, binary::Binary, call::Call, get::Get, grouping::Grouping,
            literal::Literal, logical::Logical, set::Set, super_keyword::Super, this::This,
            unary::Unary, variable::Variable, Expr,
        },
        stmt::{
            self, block::Block, class::Class, expression::Expression, function::Function,
            print::Print, r#if::If, r#while::While, var::Var, Stmt,
        },
    },
    scanner::{
        token::{Object, Token},
        token_type::TokenType,
    },
    utils::error::{Error, Return, RuntimeError},
};

struct ExprKey {
    expr: Rc<dyn Expr>,
}

impl PartialEq for ExprKey {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.expr, &other.expr)
    }
}

impl Eq for ExprKey {}

impl Hash for ExprKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.expr).hash(state);
    }
}

impl ExprKey {
    fn new(expr: Rc<dyn Expr>) -> Self {
        Self { expr }
    }
}

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
    globals: Rc<RefCell<Environment>>,
    locals: HashMap<ExprKey, u8>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new(None)));
        let environment = Rc::clone(&globals);
        let locals = HashMap::new();

        globals.borrow_mut().define(
            String::from("clock"),
            Object::Callable(Box::new(Fun::Clock(ClockFn::new()))),
        );

        Self {
            environment,
            globals,
            locals,
        }
    }

    pub fn interpret(&mut self, statements: Vec<Rc<dyn Stmt>>) {
        for statement in statements {
            if let Err(e) = self.execute(statement.as_ref()) {
                match e {
                    Error::Runtime(e) => {
                        if let crate::utils::error::Runtime::RuntimeError(error) = e {
                            runtime_error(error);
                        }
                    }
                    _ => println!("{}", e),
                }
            }
        }
    }

    fn execute_block(
        &mut self,
        statements: &Vec<Rc<dyn Stmt>>,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<Object, Error> {
        let previous = Rc::clone(&self.environment);
        self.environment = environment;
        for statement in statements {
            if let Err(e) = self.execute(statement.as_ref()) {
                self.environment = previous;
                return Err(e);
            }
        }
        self.environment = previous;
        Ok(Object::Nil)
    }

    fn evaluate(&mut self, expr: &dyn Expr) -> Result<Object, Error> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &dyn Stmt) -> Result<Object, Error> {
        stmt.accept(self)
    }

    pub fn resolve(&mut self, expr: Rc<dyn Expr>, depth: u8) -> Result<Object, Error> {
        let key = ExprKey::new(expr);
        self.locals.insert(key, depth);
        Ok(Object::Nil)
    }

    fn look_up_variable(&self, name: &Token, expr: Rc<dyn Expr>) -> Result<Object, Error> {
        let key = ExprKey::new(expr);
        if let Some(distance) = self.locals.get(&key) {
            self.environment
                .borrow()
                .get_at(*distance as usize, name.lexeme().to_owned())
        } else {
            self.globals.borrow().get(name)
        }
    }
}

impl expr::Visitor for Interpreter {
    fn visit_assign_expr(&mut self, expr: &Assign) -> Result<Object, Error> {
        let value = self.evaluate(expr.value())?;
        let key = ExprKey::new(Rc::new(expr.clone()));

        let distance = self.locals.get(&key);
        if let Some(distance) = distance {
            self.environment.borrow_mut().assign_at(
                *distance as usize,
                expr.name(),
                value.to_owned(),
            )?;
        } else {
            self.globals
                .borrow_mut()
                .assign(expr.name(), value.to_owned())?;
        }

        Ok(value)
    }

    fn visit_binary_expr(&mut self, expr: &Binary) -> Result<Object, Error> {
        let left = self.evaluate(expr.left())?;
        let right = self.evaluate(expr.right())?;

        match expr.operator().token_type() {
            TokenType::Minus => {
                if let (Object::Num(l), Object::Num(r)) = (left, right) {
                    Ok(Object::Num(l - r))
                } else {
                    Err(Error::Runtime(
                        RuntimeError::new(
                            String::from("Both operands must be numbers"),
                            expr.operator().to_owned(),
                        )
                        .into(),
                    ))
                }
            }
            TokenType::Slash => {
                if let (Object::Num(l), Object::Num(r)) = (left, right) {
                    Ok(Object::Num(l / r))
                } else {
                    Err(Error::Runtime(
                        RuntimeError::new(
                            String::from("Both operands must be numbers"),
                            expr.operator().to_owned(),
                        )
                        .into(),
                    ))
                }
            }
            TokenType::Star => {
                if let (Object::Num(l), Object::Num(r)) = (left, right) {
                    Ok(Object::Num(l * r))
                } else {
                    Err(Error::Runtime(
                        RuntimeError::new(
                            String::from("Both operands must be numbers"),
                            expr.operator().to_owned(),
                        )
                        .into(),
                    ))
                }
            }
            TokenType::Plus => match (left, right) {
                (Object::Num(l), Object::Num(r)) => Ok(Object::Num(l + r)),
                (Object::Str(l), Object::Str(r)) => Ok(Object::Str(format!("{}{}", l, r))),
                (Object::Str(l), Object::Num(r)) => Ok(Object::Str(format!("{}{}", l, r))),
                (Object::Num(l), Object::Str(r)) => Ok(Object::Str(format!("{}{}", l, r))),
                _ => Err(Error::Runtime(
                    RuntimeError::new(
                        String::from("Both operands must be numbers or strings"),
                        expr.operator().to_owned(),
                    )
                    .into(),
                )),
            },
            TokenType::Greater => {
                if let (Object::Num(l), Object::Num(r)) = (left, right) {
                    Ok(Object::Bool(l > r))
                } else {
                    Err(Error::Runtime(
                        RuntimeError::new(
                            String::from("Both operands must be numbers"),
                            expr.operator().to_owned(),
                        )
                        .into(),
                    ))
                }
            }
            TokenType::GreaterEqual => {
                if let (Object::Num(l), Object::Num(r)) = (left, right) {
                    Ok(Object::Bool(l >= r))
                } else {
                    Err(Error::Runtime(
                        RuntimeError::new(
                            String::from("Both operands must be numbers"),
                            expr.operator().to_owned(),
                        )
                        .into(),
                    ))
                }
            }
            TokenType::Less => {
                if let (Object::Num(l), Object::Num(r)) = (left, right) {
                    Ok(Object::Bool(l < r))
                } else {
                    Err(Error::Runtime(
                        RuntimeError::new(
                            String::from("Both operands must be numbers"),
                            expr.operator().to_owned(),
                        )
                        .into(),
                    ))
                }
            }
            TokenType::LessEqual => {
                if let (Object::Num(l), Object::Num(r)) = (left, right) {
                    Ok(Object::Bool(l <= r))
                } else {
                    Err(Error::Runtime(
                        RuntimeError::new(
                            String::from("Both operands must be numbers"),
                            expr.operator().to_owned(),
                        )
                        .into(),
                    ))
                }
            }
            TokenType::BangEqual => Ok(Object::Bool(!is_equal(left, right))),
            TokenType::EqualEqual => Ok(Object::Bool(is_equal(left, right))),
            _ => Ok(Object::Nil),
        }
    }

    fn visit_call_expr(&mut self, expr: &Call) -> Result<Object, Error> {
        let callee = self.evaluate(expr.callee())?;

        let arguments = expr
            .arguments()
            .iter()
            .map(|arg| self.evaluate(arg.as_ref()))
            .collect::<Result<Vec<Object>, Error>>()?;

        if let Object::Callable(callee) = callee {
            if arguments.len() != callee.arity() {
                return Err(Error::Runtime(
                    RuntimeError::new(
                        format!(
                            "Expected {} arguments but got {}.",
                            callee.arity(),
                            arguments.len()
                        ),
                        expr.paren().to_owned(),
                    )
                    .into(),
                ));
            }
            Ok(callee.call(self, arguments))
        } else {
            Err(Error::Runtime(
                RuntimeError::new(
                    String::from("Can only call functions and classes"),
                    expr.paren().to_owned(),
                )
                .into(),
            ))
        }
    }

    fn visit_get_expr(&mut self, expr: &Get) -> Result<Object, Error> {
        let object = self.evaluate(expr.object().as_ref())?;
        if let Object::Instance(instance) = object {
            instance.get(expr.name())
        } else {
            Err(Error::Runtime(
                RuntimeError::new(
                    String::from("Only instances have properties"),
                    expr.name().to_owned(),
                )
                .into(),
            ))
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<Object, Error> {
        self.evaluate(expr.expression())
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> Result<Object, Error> {
        Ok(expr.value().to_owned())
    }

    fn visit_logical_expr(&mut self, expr: &Logical) -> Result<Object, Error> {
        let left = self.evaluate(expr.left())?;

        if expr.operator().token_type() == &TokenType::Or {
            if left.is_truthy() {
                return Ok(left);
            }
        } else if !left.is_truthy() {
            return Ok(left);
        }

        self.evaluate(expr.right())
    }

    fn visit_set_expr(&mut self, expr: &Set) -> Result<Object, Error> {
        let object = self.evaluate(expr.object().as_ref())?;

        if let Object::Instance(mut instance) = object {
            let value = self.evaluate(expr.value().as_ref())?;
            instance.set(expr.name(), value.to_owned());
            Ok(value)
        } else {
            Err(Error::Runtime(
                RuntimeError::new(
                    String::from("Only instances have fields"),
                    expr.name().to_owned(),
                )
                .into(),
            ))
        }
    }

    fn visit_super_expr(&mut self, expr: &Super) -> Result<Object, Error> {
        let key = ExprKey::new(Rc::new(expr.clone()));
        let distance = self.locals.get(&key).unwrap();
        let superclass = self
            .environment
            .borrow()
            .get_at(*distance as usize, String::from("super"))?;

        let object = self
            .environment
            .borrow()
            .get_at(*distance as usize - 1, String::from("this"))?;

        let instance = match object {
            Object::Instance(instance) => Some(instance),
            _ => None,
        };

        let lexeme = expr.method().lexeme();

        let method = match superclass {
            Object::Class(class) => class.find_method(lexeme).cloned(),
            _ => None,
        };

        if let Some(method) = method {
            Ok(Object::Callable(Box::new(Fun::Function(
                method.bind(&instance.unwrap()),
            ))))
        } else {
            return Err(Error::Runtime(
                RuntimeError::new(
                    format!("Undefined property '{}'.", lexeme),
                    expr.method().to_owned(),
                )
                .into(),
            ));
        }
    }

    fn visit_this_expr(&mut self, expr: &This) -> Result<Object, Error> {
        self.look_up_variable(expr.keyword(), Rc::new(expr.clone()))
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> Result<Object, Error> {
        let right = self.evaluate(expr.right())?;

        let result = match expr.operator().token_type() {
            TokenType::Minus => {
                if let Object::Num(n) = right {
                    Ok(Object::Num(n.neg()))
                } else {
                    Err(Error::Runtime(
                        RuntimeError::new(
                            String::from("Unary minus must be applied to a number"),
                            expr.operator().to_owned(),
                        )
                        .into(),
                    ))
                }
            }
            TokenType::Bang => Ok(Object::Bool(!right.is_truthy())),
            _ => Ok(Object::Nil),
        };

        result
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> Result<Object, Error> {
        self.look_up_variable(expr.name(), Rc::new(expr.clone()))
    }
}

impl stmt::Visitor for Interpreter {
    fn visit_block_stmt(&mut self, stmt: &Block) -> Result<Object, Error> {
        let inner_environment = Environment::new(Some(Rc::clone(&self.environment)));
        let inner_environment = Rc::new(RefCell::new(inner_environment));
        self.execute_block(stmt.statements(), inner_environment)
    }

    fn visit_class_stmt(&mut self, stmt: &Class) -> Result<Object, Error> {
        let superclass = if let Some(superclass) = stmt.superclass() {
            let name = superclass.name().to_owned();
            let superclass = self.evaluate(superclass)?;
            let sc = match superclass {
                Object::Class(class) => Ok(class),
                _ => Err(Error::Runtime(
                    RuntimeError::new(String::from("Superclass must be a class."), name).into(),
                )),
            }?;
            Some(Box::new(sc))
        } else {
            None
        };

        let lexeme = stmt.name().lexeme();
        self.environment
            .borrow_mut()
            .define(lexeme.to_owned(), Object::Nil);

        if let Some(superclass) = stmt.superclass() {
            self.environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
                &self.environment,
            )))));
            self.environment.borrow_mut().define(
                String::from("super"),
                Object::Variable(Box::new(superclass.clone())),
            )
        }

        let mut methods: HashMap<String, callable::Function> = HashMap::new();
        for method in stmt.methods() {
            let is_initializer = method.name().lexeme() == "init";
            let function = callable::Function::new(
                method.clone(),
                Rc::clone(&self.environment),
                is_initializer,
            );
            methods.insert(method.name().lexeme().to_owned(), function);
        }
        let klass = callable::Class::new(lexeme.to_owned(), methods, superclass);
        let klass = Object::Class(klass);

        if stmt.superclass().is_some() {
            let enclosing = self.environment.borrow().enclosing.clone().unwrap();
            self.environment = enclosing;
        }

        self.environment.borrow_mut().assign(stmt.name(), klass)?;
        Ok(Object::Nil)
    }

    fn visit_expression_stmt(&mut self, stmt: &Expression) -> Result<Object, Error> {
        self.evaluate(stmt.expression())?;
        Ok(Object::Nil)
    }

    fn visit_function_stmt(&mut self, stmt: &Function) -> Result<Object, Error> {
        let function = callable::Function::new(stmt.clone(), Rc::clone(&self.environment), false);
        let callable = Object::Callable(Box::new(Fun::Function(function)));
        self.environment
            .borrow_mut()
            .define(stmt.name().lexeme().to_owned(), callable);
        Ok(Object::Nil)
    }

    fn visit_if_stmt(&mut self, stmt: &If) -> Result<Object, Error> {
        let condition = self.evaluate(stmt.condition())?;
        if condition.is_truthy() {
            self.execute(stmt.then_branch())?;
        } else if let Some(else_branch) = stmt.else_branch() {
            self.execute(else_branch)?;
        }

        Ok(Object::Nil)
    }

    fn visit_print_stmt(&mut self, stmt: &Print) -> Result<Object, Error> {
        let value = self.evaluate(stmt.expression())?;
        println!("{}", value);
        Ok(Object::Nil)
    }

    fn visit_return_stmt(&mut self, stmt: &stmt::r#return::Return) -> Result<Object, Error> {
        let value = if let Some(value) = stmt.value() {
            self.evaluate(value.as_ref())?
        } else {
            Object::Nil
        };

        Err(Error::Runtime(Return::new(value).into()))
    }

    fn visit_var_stmt(&mut self, stmt: &Var) -> Result<Object, Error> {
        let value = if let Some(initializer) = stmt.initializer() {
            self.evaluate(initializer.as_ref())?
        } else {
            Object::Nil
        };

        self.environment
            .borrow_mut()
            .define(stmt.name().lexeme().to_owned(), value);
        Ok(Object::Nil)
    }

    fn visit_while_stmt(&mut self, stmt: &While) -> Result<Object, Error> {
        while self.evaluate(stmt.condition())?.is_truthy() {
            self.execute(stmt.body())?;
        }

        Ok(Object::Nil)
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

fn is_equal(a: Object, b: Object) -> bool {
    if a == Object::Nil && b == Object::Nil {
        return true;
    }

    if a == Object::Nil {
        return false;
    }

    a == b
}
