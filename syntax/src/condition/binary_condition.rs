use crate::ComparisonNode;
use syntax_kind::{SyntaxKind, SyntaxNode};

#[derive(Debug)]
pub struct ComparisonConditionNode {
    pub node: SyntaxNode,
    pub parent: SyntaxNode,
}

impl ComparisonConditionNode {
    pub fn cast(node: SyntaxNode, parent: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::COMPARISON_CONDITION_NODE => Some(Self { node, parent }),
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

    pub fn comparison(&self) -> Option<ComparisonNode> {
        self.node
            .children()
            .find_map(|child| ComparisonNode::cast(child, self.syntax()))
    }
}
