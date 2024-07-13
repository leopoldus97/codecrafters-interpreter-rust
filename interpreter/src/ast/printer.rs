use super::{binary, grouping, literal, unary, Expr, Visitor};

pub struct AstPrinter {}

impl AstPrinter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print(
        &mut self,
        expr: Box<dyn Expr<String, Box<dyn std::error::Error>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        expr.accept(self)
    }

    fn parenthesize(
        &mut self,
        name: &str,
        exprs: &[&dyn Expr<String, Box<dyn std::error::Error>>],
    ) -> Result<String, Box<dyn std::error::Error>> {
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

impl Visitor<String, Box<dyn std::error::Error>> for AstPrinter {
    fn visit_binary_expr(
        &mut self,
        expr: &binary::Binary<String, Box<dyn std::error::Error>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let exprs = [expr.left(), expr.right()];
        self.parenthesize(expr.operator().lexeme(), &exprs)
    }

    fn visit_grouping_expr(
        &mut self,
        expr: &grouping::Grouping<String, Box<dyn std::error::Error>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let exprs = [expr.expression()];
        self.parenthesize("group", &exprs)
    }

    fn visit_literal_expr(
        &mut self,
        expr: &literal::Literal,
    ) -> Result<String, Box<dyn std::error::Error>> {
        Ok(expr.value().to_string())
    }

    fn visit_unary_expr(
        &mut self,
        expr: &unary::Unary<String, Box<dyn std::error::Error>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let exprs = [expr.right()];
        self.parenthesize(expr.operator().lexeme(), &exprs)
    }
}

impl Default for AstPrinter {
    fn default() -> Self {
        Self::new()
    }
}
