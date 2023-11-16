use crate::{QuestionParameterNode, SingularLiteralNode};
use syntax_kind::{SyntaxKind, SyntaxNode};

#[derive(Debug)]
pub struct BinaryComparisonNode {
    pub node: SyntaxNode,
    pub parent: SyntaxNode,
}

#[derive(Debug)]
pub struct BinaryComparisonOperatorNode {
    pub node: SyntaxNode,
    pub parent: SyntaxNode,
}

impl BinaryComparisonNode {
    pub fn cast(node: SyntaxNode, parent: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::BINARY_COMPARISON_NODE => Some(Self { node, parent }),
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

    pub fn operator(&self) -> Option<BinaryComparisonOperatorNode> {
        self.node
            .children()
            .find_map(|child| BinaryComparisonOperatorNode::cast(child, self.syntax()))
    }

    pub fn value(&self) -> Option<SingularLiteralNode> {
        self.node
            .children()
            .find_map(|child| SingularLiteralNode::cast(child, self.syntax()))
    }
}

impl BinaryComparisonOperatorNode {
    pub fn cast(node: SyntaxNode, parent: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::EQUAL_OPERATOR_NODE
            | SyntaxKind::NOT_EQUAL_OPERATOR_NODE
            | SyntaxKind::LESS_THAN_OPERATOR_NODE
            | SyntaxKind::GREATER_THAN_OPERATOR_NODE
            | SyntaxKind::LESS_THAN_OR_EQUAL_OPERATOR_NODE
            | SyntaxKind::GREATER_THAN_OR_EQUAL_OPERATOR_NODE => Some(Self { node, parent }),
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
