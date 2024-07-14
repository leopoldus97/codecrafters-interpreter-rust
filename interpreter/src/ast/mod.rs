pub mod binary;
pub mod expression;
pub mod grouping;
pub mod literal;
pub mod print;
pub mod printer;
pub mod unary;

pub mod expr {
    // expression     → equality ;
    // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    // term           → factor ( ( "-" | "+" ) factor )* ;
    // factor         → unary ( ( "/" | "*" ) unary )* ;
    // unary          → ( "!" | "-" ) unary
    //                | primary ;
    // primary        → NUMBER | STRING | "true" | "false" | "nil"
    //                | "(" expression ")" ;

    use super::{binary, grouping, literal, unary};
    
    pub trait Expr<R, E> {
        fn accept(&self, visitor: &mut dyn Visitor<R, E>) -> Result<R, E>;
    }

    pub trait Visitor<R, E> {
        fn visit_binary_expr(&mut self, expr: &binary::Binary<R, E>) -> Result<R, E>;
        fn visit_grouping_expr(&mut self, expr: &grouping::Grouping<R, E>) -> Result<R, E>;
        fn visit_literal_expr(&mut self, expr: &literal::Literal) -> Result<R, E>;
        fn visit_unary_expr(&mut self, expr: &unary::Unary<R, E>) -> Result<R, E>;
    }
}

pub mod stmt {
    // program        → statement* EOF ;
    // statement      → exprStmt
    //                | printStmt ;
    // exprStmt       → expression ";" ;
    // printStmt      → "print" expression ";" ;

    use super::{expression, print};

    pub trait Stmt<R, E> {
        fn accept(&self, visitor: &mut dyn Visitor<R, E>) -> Result<R, E>;
    }

    pub trait Visitor<R, E> {
        fn visit_expression_stmt(&mut self, stmt: &expression::Expression<R, E>) -> Result<R, E>;
        fn visit_print_stmt(&mut self, stmt: &print::Print<R, E>) -> Result<R, E>;
    }
}
