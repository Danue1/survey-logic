use crate::ConditionNode;
use syntax_kind::{SyntaxKind, SyntaxNode};

#[derive(Debug)]
pub struct GroupConditionNode {
    pub node: SyntaxNode,
    pub parent: SyntaxNode,
}

impl GroupConditionNode {
    pub fn cast(node: SyntaxNode, parent: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::GROUP_CONDITION_NODE => Some(Self { node, parent }),
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
