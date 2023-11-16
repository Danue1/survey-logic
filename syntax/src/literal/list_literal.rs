use crate::SingularLiteralNode;
use syntax_kind::{SyntaxKind, SyntaxNode};

#[derive(Debug)]
pub struct ListLiteralNode {
    pub node: SyntaxNode,
    pub parent: SyntaxNode,
}

impl ListLiteralNode {
    pub fn cast(node: SyntaxNode, parent: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::LIST_LITERAL_NODE => Some(Self { node, parent }),
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

    pub fn values(&self) -> Vec<SingularLiteralNode> {
        self.node
            .children()
            .filter_map(|child| SingularLiteralNode::cast(child, self.syntax()))
            .collect()
    }
}
