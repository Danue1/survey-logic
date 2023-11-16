use syntax_kind::SyntaxNode;

#[derive(Clone)]
pub struct NumericLiteral {
    syntax: SyntaxNode,
    parent: SyntaxNode,
    value: u64,
}

impl std::fmt::Debug for NumericLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NumericLiteral")
            .field("value", &self.value)
            .finish()
    }
}

impl NumericLiteral {
    pub const fn new(syntax: SyntaxNode, parent: SyntaxNode, value: u64) -> Self {
        Self {
            syntax,
            parent,
            value,
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

    pub const fn value(&self) -> u64 {
        self.value
    }
}
