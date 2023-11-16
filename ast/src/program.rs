use crate::Statement;
use syntax_kind::SyntaxNode;

pub struct Program {
    syntax: SyntaxNode,
    statements: Vec<Statement>,
}

impl std::fmt::Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Program")
            .field("statements", &self.statements)
            .finish()
    }
}

impl Program {
    pub const fn new(syntax: SyntaxNode, statements: Vec<Statement>) -> Self {
        Self { syntax, statements }
    }

    #[inline]
    pub fn syntax(&self) -> SyntaxNode {
        self.syntax.clone()
    }

    #[inline]
    pub fn statements(&self) -> &[Statement] {
        &self.statements
    }
}
