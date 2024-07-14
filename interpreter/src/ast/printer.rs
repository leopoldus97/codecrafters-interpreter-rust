use crate::utils::error::Error;

use super::{
    binary,
    expr::{self, Expr},
    grouping, literal, unary,
};

pub struct AstPrinter {}

impl AstPrinter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print(
        &mut self,
        expr: Box<dyn Expr<String>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        expr.accept(self).map_err(|err| err.to_string().into())
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&dyn Expr<String>]) -> Result<String, Error> {
        let mut result = String::from("(");

        result.push_str(name);
        for expr in exprs {
            result.push(' ');
            result.push_str(&expr.accept(self)?);
        }
        result.push(')');
        Ok(result)
    }
}

impl expr::Visitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &binary::Binary<String>) -> Result<String, Error> {
        let exprs = [expr.left(), expr.right()];
        self.parenthesize(expr.operator().lexeme(), &exprs)
    }

    fn visit_grouping_expr(&mut self, expr: &grouping::Grouping<String>) -> Result<String, Error> {
        let exprs = [expr.expression()];
        self.parenthesize("group", &exprs)
    }

    fn visit_literal_expr(&mut self, expr: &literal::Literal) -> Result<String, Error> {
        Ok(expr.value().to_string())
    }

    fn visit_unary_expr(&mut self, expr: &unary::Unary<String>) -> Result<String, Error> {
        let exprs = [expr.right()];
        self.parenthesize(expr.operator().lexeme(), &exprs)
    }

    fn visit_variable_expr(&mut self, expr: &super::variable::Variable) -> Result<String, Error> {
        Ok(expr.name().lexeme().to_string())
    }
}

impl Default for AstPrinter {
    fn default() -> Self {
        Self::new()
    }
}
