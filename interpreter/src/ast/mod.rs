pub mod assign;
pub mod binary;
pub mod block;
pub mod expression;
pub mod grouping;
pub mod literal;
pub mod print;
pub mod printer;
pub mod unary;
pub mod var;
pub mod variable;

pub mod expr {
    // expression     → equality ;
    // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    // term           → factor ( ( "-" | "+" ) factor )* ;
    // factor         → unary ( ( "/" | "*" ) unary )* ;
    // unary          → ( "!" | "-" ) unary
    //                | primary ;
    // primary        → NUMBER | STRING | "true" | "false" | "nil"
    //                | "(" expression ")" | IDENTIFIER ;

    use std::any::Any;

    use crate::utils::error::Error;

    use super::{assign, binary, grouping, literal, unary};

    pub trait Expr<R: 'static>: Any {
        fn accept(&self, visitor: &mut dyn Visitor<R>) -> Result<R, Error>;
        fn as_any(&self) -> &dyn Any;
    }

    pub trait Visitor<R> {
        fn visit_assign_expr(&mut self, expr: &assign::Assign<R>) -> Result<R, Error>;
        fn visit_binary_expr(&mut self, expr: &binary::Binary<R>) -> Result<R, Error>;
        fn visit_grouping_expr(&mut self, expr: &grouping::Grouping<R>) -> Result<R, Error>;
        fn visit_literal_expr(&mut self, expr: &literal::Literal) -> Result<R, Error>;
        fn visit_unary_expr(&mut self, expr: &unary::Unary<R>) -> Result<R, Error>;
        fn visit_variable_expr(&mut self, expr: &super::variable::Variable) -> Result<R, Error>;
    }
}

pub mod stmt {
    // program        → declaration* EOF ;
    // declaration    → varDecl
    //                | statement ;
    // varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
    // statement      → exprStmt
    //                | printStmt 
    //                | block ;
    // exprStmt       → expression ";" ;
    // printStmt      → "print" expression ";" ;
    // block          → "{" declaration* "}" ;

    use std::any::Any;

    use crate::utils::error::Error;

    use super::{expression, print};

    pub trait Stmt: Any {
        fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Error>;
        fn as_any(&self) -> &dyn Any;
    }

    pub trait Visitor {
        fn visit_block_stmt(&mut self, stmt: &super::block::Block) -> Result<(), Error>;
        fn visit_expression_stmt(&mut self, stmt: &expression::Expression) -> Result<(), Error>;
        fn visit_print_stmt(&mut self, stmt: &print::Print) -> Result<(), Error>;
        fn visit_var_stmt(&mut self, stmt: &super::var::Var) -> Result<(), Error>;
    }
}
