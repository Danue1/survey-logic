use crate::SyntaxResult;
use ast::GroupCondition;
use syntax::GroupConditionNode;

pub fn lower_group_condition(node: GroupConditionNode) -> SyntaxResult<GroupCondition> {
    let condition = match node.condition() {
        Some(condition) => crate::lower_condition(condition)?,
        None => invalidate!("Expected a condition"),
    };
    Ok(GroupCondition::new(node.syntax(), node.parent(), condition))
}
