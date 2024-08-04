use crate::{
    ast::{
        expr::variable::Variable,
        stmt::{function::Function, Stmt},
    },
    scanner::token::Token,
};

pub struct Class {
    name: Token,
    methods: Vec<Function>,
    superclass: Option<Variable>,
}

impl Class {
    pub fn new(name: Token, methods: Vec<Function>, superclass: Option<Variable>) -> Self {
        Self {
            name,
            methods,
            superclass,
        }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn methods(&self) -> &Vec<Function> {
        &self.methods
    }

    pub fn superclass(&self) -> &Option<Variable> {
        &self.superclass
    }
}

impl Stmt for Class {
    fn accept(
        &self,
        visitor: &mut dyn super::Visitor,
    ) -> Result<crate::scanner::token::Object, crate::utils::error::Error> {
        visitor.visit_class_stmt(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
