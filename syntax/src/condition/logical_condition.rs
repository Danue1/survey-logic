use crate::ConditionNode;
use syntax_kind::{SyntaxKind, SyntaxNode};

#[derive(Debug)]
pub struct LogicalConditionNode {
    pub node: SyntaxNode,
    pub parent: SyntaxNode,
}

#[derive(Debug)]
pub struct LogicalConditionOperatorNode {
    pub node: SyntaxNode,
    pub parent: SyntaxNode,
}

impl LogicalConditionNode {
    pub fn cast(node: SyntaxNode, parent: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::LOGICAL_CONDITION_NODE => Some(Self { node, parent }),
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

    pub fn left(&self) -> Option<ConditionNode> {
        ConditionNode::cast(self.node.children().next()?, self.syntax())
    }

    pub fn operator(&self) -> Option<LogicalConditionOperatorNode> {
        self.node
            .children()
            .find_map(|child| LogicalConditionOperatorNode::cast(child, self.syntax()))
    }

    pub fn right(&self) -> Option<ConditionNode> {
        ConditionNode::cast(self.node.children().nth(1)?, self.syntax())
    }
}

impl LogicalConditionOperatorNode {
    pub fn cast(node: SyntaxNode, parent: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::AND_OPERATOR_NODE | SyntaxKind::OR_OPERATOR_NODE => {
                Some(Self { node, parent })
            }
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
