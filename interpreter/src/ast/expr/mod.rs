pub mod assign;
pub mod binary;
pub mod call;
pub mod get;
pub mod grouping;
pub mod literal;
pub mod logical;
pub mod set;
pub mod super_keyword;
pub mod this;
pub mod unary;
pub mod variable;

// expression     → assignment ;
// assignment     → ( call "." )? IDENTIFIER "=" assignment
//                | logic_or ;
// logic_or       → logic_and ( "or" logic_and )* ;
// logic_and      → equality ( "and" equality )* ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary | call ;
// call           → primary ( "(" arguments? ")" | "." IDENTIFIER )* ;
// arguments      → expression ( "," expression )* ;
// primary        → "true" | "false" | "nil" | "this"
//                | NUMBER | STRING | IDENTIFIER | "(" expression ")"
//                | "super" "." IDENTIFIER ;

use std::any::Any;

use crate::{scanner::token::Object, utils::error::Error};

pub trait Expr: Any {
    fn accept(&self, visitor: &mut dyn Visitor) -> Result<Object, Error>;
    fn as_any(&self) -> &dyn Any;
}

pub trait Visitor {
    fn visit_assign_expr(&mut self, expr: &assign::Assign) -> Result<Object, Error>;
    fn visit_binary_expr(&mut self, expr: &binary::Binary) -> Result<Object, Error>;
    fn visit_call_expr(&mut self, expr: &call::Call) -> Result<Object, Error>;
    fn visit_get_expr(&mut self, expr: &get::Get) -> Result<Object, Error>;
    fn visit_grouping_expr(&mut self, expr: &grouping::Grouping) -> Result<Object, Error>;
    fn visit_literal_expr(&mut self, expr: &literal::Literal) -> Result<Object, Error>;
    fn visit_logical_expr(&mut self, expr: &logical::Logical) -> Result<Object, Error>;
    fn visit_set_expr(&mut self, expr: &set::Set) -> Result<Object, Error>;
    fn visit_super_expr(&mut self, expr: &super_keyword::Super) -> Result<Object, Error>;
    fn visit_this_expr(&mut self, expr: &this::This) -> Result<Object, Error>;
    fn visit_unary_expr(&mut self, expr: &unary::Unary) -> Result<Object, Error>;
    fn visit_variable_expr(&mut self, expr: &variable::Variable) -> Result<Object, Error>;
}
