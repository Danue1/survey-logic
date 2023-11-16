use syntax_kind::{SyntaxKind, SyntaxNode};

#[derive(Debug)]
pub struct NumericLiteralNode {
    pub node: SyntaxNode,
    pub parent: SyntaxNode,
}

impl NumericLiteralNode {
    pub fn cast(node: SyntaxNode, parent: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::NUMERIC_LITERAL_NODE => Some(Self { node, parent }),
            _ => None,
        }
    }

    #[inline]
    pub fn syntax(&self) -> SyntaxNode {
        self.node.clone()
    }

    #[inline]
    pub fn parent(&self) -> SyntaxNode {
        self.parent.clone()
    }
}
