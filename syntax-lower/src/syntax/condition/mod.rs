mod comparison_condition;
mod group_condition;
mod logical_condition;

use crate::SyntaxResult;
use ast::{Condition, ConditionKind};
pub use comparison_condition::*;
pub use group_condition::*;
pub use logical_condition::*;
use syntax::{ConditionKindNode, ConditionNode};

pub fn lower_condition(node: ConditionNode) -> SyntaxResult<Condition> {
    let kind = match node.kind() {
        Some(kind) => lower_condition_kind(kind)?,
        None => invalidate!("Expected a condition kind"),
    };
    Ok(Condition::new(node.syntax(), node.parent(), kind))
}

pub fn lower_condition_kind(node: ConditionKindNode) -> SyntaxResult<ConditionKind> {
    match node {
        ConditionKindNode::Group(group) => {
            let group = crate::lower_group_condition(group)?;
            Ok(ConditionKind::Group(Box::new(group)))
        }
        ConditionKindNode::Logical(logical) => {
            let logical = crate::lower_logical_condition(logical)?;
            Ok(ConditionKind::Logical(Box::new(logical)))
        }
        ConditionKindNode::Comparison(comparison) => {
            let comparison = crate::lower_comparison_condition(comparison)?;
            Ok(ConditionKind::Binary(Box::new(comparison)))
        }
    }
}
