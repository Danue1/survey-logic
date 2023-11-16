mod binary_condition;
mod group_condition;
mod logical_condition;

pub use binary_condition::*;
pub use group_condition::*;
pub use logical_condition::*;
use syntax_kind::{SyntaxKind, SyntaxNode};

#[derive(Debug)]
pub struct ConditionNode {
    pub node: SyntaxNode,
    pub parent: SyntaxNode,
}

#[derive(Debug)]
pub enum ConditionKindNode {
    Group(GroupConditionNode),
    Logical(LogicalConditionNode),
    Comparison(ComparisonConditionNode),
}

impl ConditionNode {
    pub fn cast(node: SyntaxNode, parent: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::GROUP_CONDITION_NODE
            | SyntaxKind::LOGICAL_CONDITION_NODE
            | SyntaxKind::COMPARISON_CONDITION_NODE => Some(Self { node, parent }),
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

    pub fn kind(&self) -> Option<ConditionKindNode> {
        ConditionKindNode::cast(self.node.clone(), self.syntax())
    }
}

impl ConditionKindNode {
    pub fn cast(node: SyntaxNode, parent: SyntaxNode) -> Option<Self> {
        if let Some(condition) = GroupConditionNode::cast(node.clone(), parent.clone()) {
            Some(Self::Group(condition))
        } else if let Some(condition) = LogicalConditionNode::cast(node.clone(), parent.clone()) {
            Some(Self::Logical(condition))
        } else if let Some(condition) = ComparisonConditionNode::cast(node, parent) {
            Some(Self::Comparison(condition))
        } else {
            None
        }
    }

    #[inline]
    pub fn syntax(&self) -> SyntaxNode {
        match self {
            Self::Group(condition) => condition.syntax(),
            Self::Logical(condition) => condition.syntax(),
            Self::Comparison(condition) => condition.syntax(),
        }
    }

    #[inline]
    pub fn parent(&self) -> SyntaxNode {
        match self {
            Self::Group(condition) => condition.parent(),
            Self::Logical(condition) => condition.parent(),
            Self::Comparison(condition) => condition.parent(),
        }
    }
}
