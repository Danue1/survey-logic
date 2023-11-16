use crate::SyntaxResult;
use ast::ListLiteral;
use syntax::ListLiteralNode;

pub fn lower_list_literal(node: ListLiteralNode) -> SyntaxResult<ListLiteral> {
    let mut values = vec![];
    for value in node.values() {
        values.push(crate::lower_singular_literal(value)?);
    }
    Ok(ListLiteral::new(node.syntax(), node.parent(), values))
}
