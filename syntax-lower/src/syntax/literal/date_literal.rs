use crate::SyntaxResult;
use ast::DateLiteral;
use syntax::DateLiteralNode;

pub fn lower_date_literal(node: DateLiteralNode) -> SyntaxResult<DateLiteral> {
    let value = node.syntax().text().to_string();
    Ok(DateLiteral::new(node.syntax(), node.parent(), value))
}
