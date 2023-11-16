use crate::SyntaxResult;
use ast::{SingularLiteral, SingularLiteralKind};
use syntax::{SingularLiteralKindNode, SingularLiteralNode};

pub fn lower_singular_literal(node: SingularLiteralNode) -> SyntaxResult<SingularLiteral> {
    let kind = match node.kind() {
        Some(kind) => lower_singular_literal_kind(kind)?,
        None => invalidate!("Expected a kind"),
    };
    Ok(SingularLiteral::new(node.syntax(), node.parent(), kind))
}

pub fn lower_singular_literal_kind(
    node: SingularLiteralKindNode,
) -> SyntaxResult<SingularLiteralKind> {
    match node {
        SingularLiteralKindNode::String(node) => {
            let literal = crate::lower_string_literal(node);
            Ok(SingularLiteralKind::String(literal))
        }
        SingularLiteralKindNode::Numeric(node) => {
            let literal = crate::lower_numeric_literal(node)?;
            Ok(SingularLiteralKind::Numeric(literal))
        }
    }
}
