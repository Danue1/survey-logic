mod binary_comparison;
mod date_comparison;
mod list_comparison;

pub use binary_comparison::*;
pub use date_comparison::*;
pub use list_comparison::*;
use syntax_kind::SyntaxNode;

#[derive(Clone)]
pub struct Comparison {
    syntax: SyntaxNode,
    parent: SyntaxNode,
    kind: ComparisonKind,
}

#[derive(Debug, Clone)]
pub enum ComparisonKind {
    Binary(BinaryComparison),
    List(ListComparison),
    Date(DateComparison),
}

impl std::fmt::Debug for Comparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Comparison")
            .field("kind", &self.kind)
            .finish()
    }
}

impl Comparison {
    pub const fn new(syntax: SyntaxNode, parent: SyntaxNode, kind: ComparisonKind) -> Self {
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

    pub const fn kind(&self) -> &ComparisonKind {
        &self.kind
    }
}
