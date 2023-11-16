use crate::Condition;
use syntax_kind::SyntaxNode;

pub struct Statement {
    syntax: SyntaxNode,
    parent: SyntaxNode,
    condition: Condition,
}

impl std::fmt::Debug for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Statement")
            .field("condition", &self.condition)
            .finish()
    }
}

impl Statement {
    pub const fn new(syntax: SyntaxNode, parent: SyntaxNode, condition: Condition) -> Self {
        Self {
            syntax,
            parent,
            condition,
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

    pub const fn condition(&self) -> &Condition {
        &self.condition
    }
}
