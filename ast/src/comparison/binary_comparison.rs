use crate::{QuestionParameter, SingularLiteral};
use syntax_kind::SyntaxNode;

#[derive(Clone)]
pub struct BinaryComparison {
    syntax: SyntaxNode,
    parent: SyntaxNode,
    parameter: QuestionParameter,
    operator: BinaryComparisonOperator,
    value: SingularLiteral,
}

#[derive(Clone)]
pub struct BinaryComparisonOperator {
    syntax: SyntaxNode,
    parent: SyntaxNode,
    kind: BinaryComparisonOperatorKind,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryComparisonOperatorKind {
    Equal,              // =
    NotEqual,           // !=
    LessThan,           // <
    GreaterThan,        // >
    LessThanOrEqual,    // <=
    GreaterThanOrEqual, // >=
}

impl std::fmt::Debug for BinaryComparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinaryComparison")
            .field("parameter", &self.parameter)
            .field("operator", &self.operator)
            .field("value", &self.value)
            .finish()
    }
}

impl std::fmt::Debug for BinaryComparisonOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinaryComparisonOperator")
            .field("kind", &self.kind)
            .finish()
    }
}

impl BinaryComparison {
    pub const fn new(
        syntax: SyntaxNode,
        parent: SyntaxNode,
        parameter: QuestionParameter,
        operator: BinaryComparisonOperator,
        value: SingularLiteral,
    ) -> Self {
        Self {
            syntax,
            parent,
            parameter,
            operator,
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

    pub const fn operator(&self) -> &BinaryComparisonOperator {
        &self.operator
    }

    pub const fn value(&self) -> &SingularLiteral {
        &self.value
    }
}

impl BinaryComparisonOperator {
    pub const fn new(
        syntax: SyntaxNode,
        parent: SyntaxNode,
        kind: BinaryComparisonOperatorKind,
    ) -> Self {
        Self {
            syntax,
            parent,
            kind,
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

    pub const fn kind(&self) -> &BinaryComparisonOperatorKind {
        &self.kind
    }
}
