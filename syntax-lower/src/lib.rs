#[macro_use]
pub mod error;
mod syntax;

use ::syntax::ProgramNode;
use ast::Program;
pub use error::*;
pub use syntax::*;
use syntax_kind::SyntaxNode;

pub fn lower(node: SyntaxNode) -> SyntaxResult<Program> {
    match ProgramNode::cast(node) {
        Some(node) => lower_program(node),
        None => invalidate!("Expected a program node"),
    }
}
