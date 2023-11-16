use crate::NumericLiteral;
use syntax_kind::SyntaxNode;

#[derive(Clone)]
pub struct QuestionParameter {
    syntax: SyntaxNode,
    parent: SyntaxNode,
    id: NumericLiteral,
}

impl std::fmt::Debug for QuestionParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QuestionParameter")
            .field("id", &self.id)
            .finish()
    }
}

impl QuestionParameter {
    pub const fn new(syntax: SyntaxNode, parent: SyntaxNode, id: NumericLiteral) -> Self {
        Self { syntax, parent, id }
    }

    #[inline]
    pub fn syntax(&self) -> SyntaxNode {
        self.syntax.clone()
    }

    #[inline]
    pub fn parent(&self) -> SyntaxNode {
        self.parent.clone()
    }

    pub const fn id(&self) -> &NumericLiteral {
        &self.id
    }
}
