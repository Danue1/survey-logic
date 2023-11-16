use crate::{DateLiteral, QuestionParameter};
use syntax_kind::SyntaxNode;

#[derive(Clone)]
pub struct DateComparison {
    syntax: SyntaxNode,
    parent: SyntaxNode,
    parameter: QuestionParameter,
    value: DateLiteral,
}

impl std::fmt::Debug for DateComparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DateComparison")
            .field("parameter", &self.parameter)
            .field("value", &self.value)
            .finish()
    }
}

impl DateComparison {
    pub const fn new(
        syntax: SyntaxNode,
        parent: SyntaxNode,
        parameter: QuestionParameter,
        value: DateLiteral,
    ) -> Self {
        Self {
            syntax,
            parent,
            parameter,
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

    pub const fn parameter(&self) -> &QuestionParameter {
        &self.parameter
    }

    pub const fn value(&self) -> &DateLiteral {
        &self.value
    }
}
