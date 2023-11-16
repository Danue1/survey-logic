use crate::Condition;
use syntax_kind::SyntaxNode;

#[derive(Clone)]
pub struct GroupCondition {
    syntax: SyntaxNode,
    parent: SyntaxNode,
    condition: Condition,
}

impl std::fmt::Debug for GroupCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GroupCondition")
            .field("condition", &self.condition)
            .finish()
    }
}

impl GroupCondition {
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
