pub mod block;
pub mod expression;
pub mod function;
pub mod r#if;
pub mod print;
pub mod r#return;
pub mod var;
pub mod r#while;

// program        → declaration* EOF ;
// declaration    → funDecl
//                | varDecl
//                | statement ;
// funDecl        → "fun" function ;
// function       → IDENTIFIER "(" parameters? ")" block ;
// parameters     → IDENTIFIER ( "," IDENTIFIER )* ;
// varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
// statement      → exprStmt
//                | forStmt
//                | ifStmt
//                | printStmt
//                | returnStmt
//                | whileStmt
//                | block ;
// forStmt        → "for" "(" ( varDecl | exprStmt | ";" )
//                 expression? ";"
//                 expression? ")" statement ;
// exprStmt       → expression ";" ;
// ifStmt         → "if" "(" expression ")" statement
//                | ( "else" statement )? ;
// printStmt      → "print" expression ";" ;
// returnStmt     → "return" expression? ";" ;
// whileStmt      → "while" "(" expression ")" statement ;
// block          → "{" declaration* "}" ;

use std::any::Any;

use crate::utils::error::Error;

pub trait Stmt: Any {
    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Error>;
    fn as_any(&self) -> &dyn Any;
}

pub trait Visitor {
    fn visit_block_stmt(&mut self, stmt: &block::Block) -> Result<(), Error>;
    fn visit_expression_stmt(&mut self, stmt: &expression::Expression) -> Result<(), Error>;
    fn visit_function_stmt(&mut self, stmt: &function::Function) -> Result<(), Error>;
    fn visit_if_stmt(&mut self, stmt: &r#if::If) -> Result<(), Error>;
    fn visit_print_stmt(&mut self, stmt: &print::Print) -> Result<(), Error>;
    fn visit_return_stmt(&mut self, stmt: &r#return::Return) -> Result<(), Error>;
    fn visit_var_stmt(&mut self, stmt: &var::Var) -> Result<(), Error>;
    fn visit_while_stmt(&mut self, stmt: &r#while::While) -> Result<(), Error>;
}
