use crate::{ListLiteral, QuestionParameter};
use syntax_kind::SyntaxNode;

#[derive(Clone)]
pub struct ListComparison {
    syntax: SyntaxNode,
    parent: SyntaxNode,
    parameter: QuestionParameter,
    list: ListLiteral,
}

impl std::fmt::Debug for ListComparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListComparison")
            .field("parameter", &self.parameter)
            .field("list", &self.list)
            .finish()
    }
}

impl ListComparison {
    pub const fn new(
        syntax: SyntaxNode,
        parent: SyntaxNode,
        parameter: QuestionParameter,
        list: ListLiteral,
    ) -> Self {
        Self {
            syntax,
            parent,
            parameter,
            list,
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

    pub const fn parameter(&self) -> &QuestionParameter {
        &self.parameter
    }

    pub const fn list(&self) -> &ListLiteral {
        &self.list
    }
}
