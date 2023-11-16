use ast::StringLiteral;
use syntax::StringLiteralNode;

pub fn lower_string_literal(node: StringLiteralNode) -> StringLiteral {
    let value = node.syntax().text().to_string();
    StringLiteral::new(node.syntax(), node.parent(), value)
}
