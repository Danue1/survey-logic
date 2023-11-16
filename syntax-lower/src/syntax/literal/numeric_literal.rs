use crate::SyntaxResult;
use ast::NumericLiteral;
use syntax::NumericLiteralNode;

pub fn lower_numeric_literal(node: NumericLiteralNode) -> SyntaxResult<NumericLiteral> {
    let value = node.syntax().text().to_string();
    let value = match value.parse() {
        Ok(value) => value,
        Err(error) => invalidate!("{:#?}", error),
    };
    Ok(NumericLiteral::new(node.syntax(), node.parent(), value))
}
