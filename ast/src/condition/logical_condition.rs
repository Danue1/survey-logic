use crate::Condition;
use syntax_kind::SyntaxNode;

#[derive(Clone)]
pub struct LogicalCondition {
    syntax: SyntaxNode,
    parent: SyntaxNode,
    left: Condition,
    operator: LogicalConditionOperator,
    right: Condition,
}

#[derive(Clone)]
pub struct LogicalConditionOperator {
    syntax: SyntaxNode,
    parent: SyntaxNode,
    kind: LogicalConditionOperatorKind,
}

#[derive(Debug, Clone, Copy)]
pub enum LogicalConditionOperatorKind {
    And,
    Or,
}

impl std::fmt::Debug for LogicalCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogicalCondition")
            .field("left", &self.left)
            .field("operator", &self.operator)
            .field("right", &self.right)
            .finish()
    }
}

impl std::fmt::Debug for LogicalConditionOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogicalConditionOperator")
            .field("kind", &self.kind)
            .finish()
    }
}

impl LogicalCondition {
    pub const fn new(
        syntax: SyntaxNode,
        parent: SyntaxNode,
        left: Condition,
        operator: LogicalConditionOperator,
        right: Condition,
    ) -> Self {
        Self {
            syntax,
            parent,
            left,
            operator,
            right,
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

    pub const fn left(&self) -> &Condition {
        &self.left
    }

    pub const fn operator(&self) -> &LogicalConditionOperator {
        &self.operator
    }

    pub const fn right(&self) -> &Condition {
        &self.right
    }
}

impl LogicalConditionOperator {
    pub const fn new(
        syntax: SyntaxNode,
        parent: SyntaxNode,
        kind: LogicalConditionOperatorKind,
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

    pub const fn kind(&self) -> &LogicalConditionOperatorKind {
        &self.kind
    }
}
