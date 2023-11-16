use crate::NumericLiteralNode;
use syntax_kind::{SyntaxKind, SyntaxNode};

#[derive(Debug)]
pub struct QuestionParameterNode {
    pub node: SyntaxNode,
    pub parent: SyntaxNode,
}

impl QuestionParameterNode {
    pub fn cast(node: SyntaxNode, parent: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::QUESTION_PARAMETER_NODE => Some(Self { node, parent }),
            _ => None,
        }
    }

    pub fn id(&self) -> Option<NumericLiteralNode> {
        self.node
            .children()
            .find_map(|child| NumericLiteralNode::cast(child, self.node.clone()))
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
