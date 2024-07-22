pub mod assign;
pub mod binary;
pub mod call;
pub mod grouping;
pub mod literal;
pub mod logical;
pub mod unary;
pub mod variable;

// expression     → assignment ;
// assignment     → IDENTIFIER "=" assignment
//                | logic_or ;
// logic_or       → logic_and ( "or" logic_and )* ;
// logic_and      → equality ( "and" equality )* ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary | call ;
// call           → primary ( "(" arguments? ")" )* ;
// arguments      → expression ( "," expression )* ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")" | IDENTIFIER ;

use std::any::Any;

use crate::utils::error::Error;

pub trait Expr<R: 'static>: Any {
    fn accept(&self, visitor: &mut dyn Visitor<R>) -> Result<R, Error>;
    fn as_any(&self) -> &dyn Any;
}

pub trait Visitor<R> {
    fn visit_assign_expr(&mut self, expr: &assign::Assign<R>) -> Result<R, Error>;
    fn visit_binary_expr(&mut self, expr: &binary::Binary<R>) -> Result<R, Error>;
    fn visit_call_expr(&mut self, expr: &call::Call<R>) -> Result<R, Error>;
    fn visit_grouping_expr(&mut self, expr: &grouping::Grouping<R>) -> Result<R, Error>;
    fn visit_literal_expr(&mut self, expr: &literal::Literal) -> Result<R, Error>;
    fn visit_logical_expr(&mut self, expr: &logical::Logical<R>) -> Result<R, Error>;
    fn visit_unary_expr(&mut self, expr: &unary::Unary<R>) -> Result<R, Error>;
    fn visit_variable_expr(&mut self, expr: &variable::Variable) -> Result<R, Error>;
}