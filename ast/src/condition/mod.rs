mod comparison_condition;
mod group_condition;
mod logical_condition;

pub use comparison_condition::*;
pub use group_condition::*;
pub use logical_condition::*;
use syntax_kind::SyntaxNode;

#[derive(Clone)]
pub struct Condition {
    syntax: SyntaxNode,
    parent: SyntaxNode,
    kind: ConditionKind,
}

#[derive(Debug, Clone)]
pub enum ConditionKind {
    Group(Box<GroupCondition>),
    Logical(Box<LogicalCondition>),
    Binary(Box<ComparisonCondition>),
}

impl std::fmt::Debug for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Condition")
            .field("kind", &self.kind)
            .finish()
    }
}

impl Condition {
    pub const fn new(syntax: SyntaxNode, parent: SyntaxNode, kind: ConditionKind) -> Self {
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

    pub const fn kind(&self) -> &ConditionKind {
        &self.kind
    }
}
