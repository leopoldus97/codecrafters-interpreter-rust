pub mod binary;
pub mod grouping;
pub mod literal;
pub mod printer;
pub mod unary;

pub trait Expr<R> {
    fn accept(&self, visitor: &mut dyn Visitor<R>) -> R;
}

pub trait Visitor<R> {
    fn visit_binary_expr(&mut self, binary: &binary::Binary<R>) -> R;
    fn visit_grouping_expr(&mut self, grouping: &grouping::Grouping<R>) -> R;
    fn visit_literal_expr(&mut self, literal: &literal::Literal) -> R;
    fn visit_unary_expr(&mut self, unary: &unary::Unary<R>) -> R;
}
