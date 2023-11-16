mod comparison;
mod condition;
mod literal;
mod program;
mod statement;

pub use comparison::*;
pub use condition::*;
pub use literal::*;
pub use program::*;
pub use statement::*;
use syntax_kind::SyntaxNode;

#[inline]
pub fn cast(node: SyntaxNode) -> Option<ProgramNode> {
    ProgramNode::cast(node)
}
