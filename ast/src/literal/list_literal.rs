use crate::SingularLiteral;
use syntax_kind::SyntaxNode;

#[derive(Clone)]
pub struct ListLiteral {
    syntax: SyntaxNode,
    parent: SyntaxNode,
    values: Vec<SingularLiteral>,
}

impl std::fmt::Debug for ListLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListLiteral")
            .field("values", &self.values)
            .finish()
    }
}

impl ListLiteral {
    pub const fn new(syntax: SyntaxNode, parent: SyntaxNode, values: Vec<SingularLiteral>) -> Self {
        Self {
            syntax,
            parent,
            values,
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

    #[inline]
    pub fn values(&self) -> &[SingularLiteral] {
        &self.values
    }
}
