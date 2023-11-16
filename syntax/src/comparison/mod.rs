mod binary_comparison;
mod date_comparison;
mod list_comparison;

pub use binary_comparison::*;
pub use date_comparison::*;
pub use list_comparison::*;
use syntax_kind::{SyntaxKind, SyntaxNode};

#[derive(Debug)]
pub struct ComparisonNode {
    pub node: SyntaxNode,
    pub parent: SyntaxNode,
}

#[derive(Debug)]
pub enum ComparisonKindNode {
    Binary(BinaryComparisonNode),
    List(ListComparisonNode),
    Date(DateComparisonNode),
}

impl ComparisonNode {
    pub fn cast(node: SyntaxNode, parent: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::BINARY_COMPARISON_NODE
            | SyntaxKind::LIST_COMPARISON_NODE
            | SyntaxKind::DATE_COMPARISON_NODE => Some(Self { node, parent }),
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

    pub fn kind(&self) -> Option<ComparisonKindNode> {
        ComparisonKindNode::cast(self.syntax(), self.syntax())
    }
}

impl ComparisonKindNode {
    pub fn cast(node: SyntaxNode, parent: SyntaxNode) -> Option<Self> {
        if let Some(comparison) = BinaryComparisonNode::cast(node.clone(), parent.clone()) {
            Some(Self::Binary(comparison))
        } else if let Some(comparison) = ListComparisonNode::cast(node.clone(), parent.clone()) {
            Some(Self::List(comparison))
        } else if let Some(comparison) = DateComparisonNode::cast(node, parent) {
            Some(Self::Date(comparison))
        } else {
            None
        }
    }

    #[inline]
    pub fn syntax(&self) -> SyntaxNode {
        match self {
            Self::Binary(comparison) => comparison.syntax(),
            Self::List(comparison) => comparison.syntax(),
            Self::Date(comparison) => comparison.syntax(),
        }
    }

    #[inline]
    pub fn parent(&self) -> SyntaxNode {
        match self {
            Self::Binary(comparison) => comparison.parent(),
            Self::List(comparison) => comparison.parent(),
            Self::Date(comparison) => comparison.parent(),
        }
    }
}
