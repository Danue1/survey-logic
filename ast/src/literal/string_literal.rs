use syntax_kind::SyntaxNode;

#[derive(Clone)]
pub struct StringLiteral {
    syntax: SyntaxNode,
    parent: SyntaxNode,
    value: String,
}

impl std::fmt::Debug for StringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StringLiteral")
            .field("value", &self.value)
            .finish()
    }
}

impl StringLiteral {
    pub const fn new(syntax: SyntaxNode, parent: SyntaxNode, value: String) -> Self {
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

    #[inline]
    pub fn value(&self) -> &str {
        &self.value
    }
}
