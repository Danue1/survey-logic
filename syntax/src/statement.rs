use crate::ConditionNode;
use syntax_kind::{SyntaxKind, SyntaxNode};

#[derive(Debug)]
pub struct StatementNode {
    node: SyntaxNode,
    parent: SyntaxNode,
}

impl StatementNode {
    pub fn cast(node: SyntaxNode, parent: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::STATEMENT_NODE => Some(Self { node, parent }),
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

    pub fn condition(&self) -> Option<ConditionNode> {
        self.node
            .children()
            .find_map(|child| ConditionNode::cast(child, self.syntax()))
    }
}
