pub mod binary;
pub mod grouping;
pub mod literal;
pub mod printer;
pub mod unary;

pub trait Expr<R, E> {
    fn accept(&self, visitor: &mut dyn Visitor<R, E>) -> Result<R, E>;
}

pub trait Visitor<R, E> {
    fn visit_binary_expr(&mut self, expr: &binary::Binary<R, E>) -> Result<R, E>;
    fn visit_grouping_expr(&mut self, expr: &grouping::Grouping<R, E>) -> Result<R, E>;
    fn visit_literal_expr(&mut self, expr: &literal::Literal) -> Result<R, E>;
    fn visit_unary_expr(&mut self, expr: &unary::Unary<R, E>) -> Result<R, E>;
}
