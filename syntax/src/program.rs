use crate::StatementNode;
use syntax_kind::{SyntaxKind, SyntaxNode};

#[derive(Debug)]
pub struct ProgramNode {
    pub node: SyntaxNode,
}

impl ProgramNode {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::PROGRAM_NODE => Some(Self { node }),
            _ => None,
        }
    }

    #[inline]
    pub fn syntax(&self) -> SyntaxNode {
        self.node.clone()
    }

    pub fn statements(&self) -> Vec<StatementNode> {
        self.node
            .children()
            .filter_map(|child| StatementNode::cast(child, self.syntax()))
            .collect()
    }
}
