mod binary_comparison;
mod date_comparison;
mod list_comparison;

use crate::SyntaxResult;
use ast::{Comparison, ComparisonKind};
pub use binary_comparison::*;
pub use date_comparison::*;
pub use list_comparison::*;
use syntax::{ComparisonKindNode, ComparisonNode};

pub fn lower_comparison(node: ComparisonNode) -> SyntaxResult<Comparison> {
    let kind = match node.kind() {
        Some(kind) => lower_comparison_kind(kind)?,
        None => invalidate!("Expected a comparison kind"),
    };
    Ok(Comparison::new(node.syntax(), node.parent(), kind))
}

pub fn lower_comparison_kind(node: ComparisonKindNode) -> SyntaxResult<ComparisonKind> {
    match node {
        ComparisonKindNode::Binary(node) => {
            let comparison = crate::lower_binary_comparison(node)?;
            Ok(ComparisonKind::Binary(comparison))
        }
        ComparisonKindNode::List(node) => {
            let comparison = crate::lower_list_comparison(node)?;
            Ok(ComparisonKind::List(comparison))
        }
        ComparisonKindNode::Date(node) => {
            let comparison = crate::lower_date_comparison(node)?;
            Ok(ComparisonKind::Date(comparison))
        }
    }
}
