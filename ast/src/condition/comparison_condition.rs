use crate::Comparison;
use syntax_kind::SyntaxNode;

#[derive(Clone)]
pub struct ComparisonCondition {
    syntax: SyntaxNode,
    parent: SyntaxNode,
    comparison: Comparison,
}

impl std::fmt::Debug for ComparisonCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComparisonCondition")
            .field("comparison", &self.comparison)
            .finish()
    }
}

impl ComparisonCondition {
    pub const fn new(syntax: SyntaxNode, parent: SyntaxNode, comparison: Comparison) -> Self {
        Self {
            syntax,
            parent,
            comparison,
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

    pub const fn comparison(&self) -> &Comparison {
        &self.comparison
    }
}
