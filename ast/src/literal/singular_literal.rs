use crate::{NumericLiteral, StringLiteral};
use syntax_kind::SyntaxNode;

#[derive(Clone)]
pub struct SingularLiteral {
    syntax: SyntaxNode,
    parent: SyntaxNode,
    kind: SingularLiteralKind,
}

#[derive(Debug, Clone)]
pub enum SingularLiteralKind {
    Numeric(NumericLiteral),
    String(StringLiteral),
}

impl std::fmt::Debug for SingularLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SingularLiteral")
            .field("kind", &self.kind)
            .finish()
    }
}

impl SingularLiteral {
    pub const fn new(syntax: SyntaxNode, parent: SyntaxNode, kind: SingularLiteralKind) -> Self {
        Self {
            syntax,
            parent,
            kind,
        }
    }

    #[inline]
    pub fn syntax(&self) -> SyntaxNode {
        self.syntax.clone()
    }

    #[inline]
    pub fn parent(&self) -> SyntaxNode {
        self.parent.clone()
    }

    pub const fn kind(&self) -> &SingularLiteralKind {
        &self.kind
    }
}
