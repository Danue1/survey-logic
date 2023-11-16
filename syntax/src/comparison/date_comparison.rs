use crate::{DateLiteralNode, QuestionParameterNode};
use syntax_kind::{SyntaxKind, SyntaxNode};

#[derive(Debug)]
pub struct DateComparisonNode {
    pub node: SyntaxNode,
    pub parent: SyntaxNode,
}

impl DateComparisonNode {
    pub fn cast(node: SyntaxNode, parent: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::DATE_COMPARISON_NODE => Some(Self { node, parent }),
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

    pub fn value(&self) -> Option<DateLiteralNode> {
        self.node
            .children()
            .find_map(|child| DateLiteralNode::cast(child, self.syntax()))
    }
}
