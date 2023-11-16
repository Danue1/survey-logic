use syntax_kind::{SyntaxKind, SyntaxNode};

#[derive(Debug)]
pub struct StringLiteralNode {
    pub node: SyntaxNode,
    pub parent: SyntaxNode,
}

impl StringLiteralNode {
    pub fn cast(node: SyntaxNode, parent: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::STRING_LITERAL_NODE => Some(Self { node, parent }),
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
