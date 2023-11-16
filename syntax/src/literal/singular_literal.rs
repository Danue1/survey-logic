use crate::{NumericLiteralNode, StringLiteralNode};
use syntax_kind::{SyntaxKind, SyntaxNode};

#[derive(Debug)]
pub struct SingularLiteralNode {
    pub node: SyntaxNode,
    pub parent: SyntaxNode,
}

#[derive(Debug)]
pub enum SingularLiteralKindNode {
    Numeric(NumericLiteralNode),
    String(StringLiteralNode),
}

impl SingularLiteralNode {
    pub fn cast(node: SyntaxNode, parent: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::NUMERIC_LITERAL_NODE | SyntaxKind::STRING_LITERAL_NODE => {
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

    pub fn kind(&self) -> Option<SingularLiteralKindNode> {
        SingularLiteralKindNode::cast(self.syntax(), self.syntax())
    }
}

impl SingularLiteralKindNode {
    pub fn cast(node: SyntaxNode, parent: SyntaxNode) -> Option<Self> {
        if let Some(literal) = NumericLiteralNode::cast(node.clone(), parent.clone()) {
            Some(Self::Numeric(literal))
        } else if let Some(literal) = StringLiteralNode::cast(node, parent) {
            Some(Self::String(literal))
        } else {
            None
        }
    }

    #[inline]
    pub fn syntax(&self) -> SyntaxNode {
        match self {
            Self::Numeric(literal) => literal.syntax(),
            Self::String(literal) => literal.syntax(),
        }
    }

    #[inline]
    pub fn parent(&self) -> SyntaxNode {
        match self {
            Self::Numeric(literal) => literal.parent(),
            Self::String(literal) => literal.parent(),
        }
    }
}
