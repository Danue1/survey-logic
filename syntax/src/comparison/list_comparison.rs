use crate::{ListLiteralNode, QuestionParameterNode};
use syntax_kind::{SyntaxKind, SyntaxNode};

#[derive(Debug)]
pub struct ListComparisonNode {
    pub node: SyntaxNode,
    pub parent: SyntaxNode,
}

impl ListComparisonNode {
    pub fn cast(node: SyntaxNode, parent: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::LIST_COMPARISON_NODE => Some(Self { node, parent }),
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

    pub fn parameter(&self) -> Option<QuestionParameterNode> {
        self.node
            .children()
            .find_map(|child| QuestionParameterNode::cast(child, self.syntax()))
    }

    pub fn list(&self) -> Option<ListLiteralNode> {
        self.node
            .children()
            .find_map(|child| ListLiteralNode::cast(child, self.syntax()))
    }
}
